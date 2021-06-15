use super::{Env, LValue};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn define_prelude(env: &mut Env) {
    env.def(
        "print".into(),
        &LValue::Lambda2(Box::new(|args| {
            let string = args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            print!("{}", string);

            // TODO: return array, if multiple args
            args.first().unwrap_or(&LValue::Bool(false)).clone()
        })),
    );

    env.def(
        "puts".into(),
        &LValue::Lambda2(Box::new(|args| {
            let string = args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            println!("{}", string);

            // TODO: return array, if multiple args
            args.first().unwrap_or(&LValue::Bool(false)).clone()
        })),
    );

    env.def(
        "sleep".into(),
        &LValue::Lambda2(Box::new(|args| match args.first() {
            Some(LValue::Num(time)) => {
                let seconds = *time as u64;
                sleep(Duration::new(seconds, 0));

                LValue::Num(seconds as f64)
            }
            _ => {
                println!("Invalid argument: must be a number.");

                LValue::Bool(false)
            }
        })),
    );

    env.def(
        "time".into(),
        &LValue::Lambda2(Box::new(|args| match args.first() {
            Some(LValue::Lambda2(lambda)) => {
                let now = Instant::now();

                lambda(vec![]);

                println!("{}", now.elapsed().as_secs());

                LValue::Bool(true)
            }
            Some(LValue::Lambda(lambda)) => {
                let now = Instant::now();

                lambda.call(vec![]).ok();

                println!("{}µs", now.elapsed().as_micros());

                LValue::Bool(true)
            }
            other => {
                println!("{:?}", other);
                LValue::Bool(false)
            }
        })),
    );
}
