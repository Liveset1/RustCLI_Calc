use std::env;

fn add(a: f32, b: f32) -> f32 {
    return a + b;
}

fn mul(a: f32, b: f32) -> f32 {
    return a * b;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let operation = &args[1];
        let mut numbers: Vec<f32> = Vec::new();

        // Operations
        let is_add = operation.eq("add") || operation.eq("+"); // Is Add
        let is_mul = operation.eq("mult") || operation.eq("*") || operation.eq("mul"); // Is Multiply

        let mut _sum: f32 = 0.0;

        if !is_add {
            _sum = 1.0;
        }

        for n in 2..args.len() {
            let num_str = &args[n];

            let num = num_str
                .parse::<f32>()
                .expect(&format!("Failed to parse float at arg ({}, value: {})", n.to_string(), num_str));
            
            // Operations
            if is_add {
                _sum = add(_sum, num);
            } else if is_mul{
                _sum = mul(_sum, num);
            }

            numbers.push(num);
        }

        let mut equation = String::new();
        equation.push_str(&format!("{}", numbers[0].to_string()));
        for (index, num) in numbers.iter().enumerate() {
            if index > 0 {
                if index < numbers.len() {
                    // Operations
                    if is_add {
                        equation.push_str(&format!(" + {}", num));
                    } else if is_mul {
                        equation.push_str(&format!(" * {}", num));
                    }
                }
            }
        }
        equation.push_str(&format!(" = {}", _sum));
        println!("{}", equation);
    } else {
        println!("No arguments provided!");
    }
}
