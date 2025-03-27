use hound::{WavReader};

const ALTITUDE_THRESHOLD: f32 = 200.0; 

fn parse_main(arr: Vec<i32>, limit: i32) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut arr = arr;
    arr.sort();
    arr.dedup();
    
    let mut current = 1;
    let mut last = 0;
    
    for num in arr.into_iter() {
        if current >= limit {
            break
        }
        if num - last < 500 || last == 0 {
            last = num;
            continue;
        }

        if numbers.len() < 2 {
            numbers.push(last);
            numbers.push(num);
        } else {
            numbers[1] = num;
        }

        last = num;
        current += 1;
    }

    numbers
}

fn detect_spaces(silence: Vec<i32>, dit: i32) -> [i32; 2] {
    let silence = parse_main(silence, 3);

    match silence.len() {
        2 => silence.try_into().unwrap(),
        1 => {
            let result = silence[0] / dit;
            if result < 3 { 
                [silence[0], 0]
            } else { 
                [0, silence[0]]
            }
        },
        _ => [0; 2]
    }
}

fn detect_peaks(samples: &[i16]) -> (Vec<(i32, bool)>, [i32; 2], [i32; 2]) {
    let mut data = Vec::new();
    let mut peak = Vec::new();
    let mut silence = Vec::new();
    
    let mut duration = 0;
    let mut current_peak = false;

    for i in 0..samples.len()-1 {
        let curr = samples[i] as f32;
        let cond = curr.abs() > ALTITUDE_THRESHOLD;
        
        if cond != current_peak && ((samples[i+1] as f32).abs() > ALTITUDE_THRESHOLD) != current_peak {
            if duration > 100 {
                if current_peak {
                    peak.push(duration)
                } else {
                    silence.push(duration)
                }
                
                data.push((duration, current_peak));
            }
        
            current_peak = cond;
            duration = 0;
        }

        duration += 1;
    }
    
    peak = parse_main(peak, 10);
    let dit = *peak.iter().min().unwrap();
    let dah = *peak.iter().max().unwrap();

    (data, [dit, dah], detect_spaces(silence, dit))
}

pub fn input() -> String {
    let mut reader = WavReader::open("morse_code.wav").unwrap();
    let samples: Vec<i16> = reader.samples::<i16>().collect::<Result<_, _>>().unwrap();

    let (data, [_dit, dah], spaces) = detect_peaks(&samples);
    let mut morse_code = String::new();

    for (val, item) in data {      
        if item {
            morse_code += if val >= dah { "-" } else { "." };
            continue;
        }

        morse_code += match val {
            val if val >= spaces[1] => " / ",
            val if val <= spaces[0] => "",
            _ => " "
        };
    }

    morse_code
}
