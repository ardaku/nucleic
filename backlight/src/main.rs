// use std::process::Command;
use std::{ env, fs, process, io };
use std::io::{ Write };
use std::path::PathBuf;

/*fn run(cmd: &str, args: &[&str]) {
	let mut builder = Command::new(cmd);
	for arg in args {
		builder.arg(arg);
	}
	builder.spawn().expect("failed to execute process")
		.wait().expect("command wasn't running");
}*/

fn read_light(light_max: f32, light: &PathBuf) -> std::io::Result<f32> {
    // Read Light
    Ok(fs::read_to_string(light)?.lines().next().unwrap()
        .parse::<i32>().unwrap() as f32 * 100.0 / light_max)
}

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		println!("Usage:\n\tbacklight 50\t\tSet to 50%\n\tbacklight +\t\tIncrease Brightness\n\tbacklight -\t\tDecrease Brightess");
		process::exit(0);
	}

	// Get Light
	let lights: io::Result<Vec<fs::DirEntry>>
		= fs::read_dir("/sys/class/backlight/")?.collect();
	let lights = lights.unwrap();
	if lights.len() != 1 {
		println!("Number of backlights is not 1, it is {}",
			lights.len());
		process::exit(-1);
	}
	let mut maxlt = lights[0].path();
	maxlt.push("max_brightness");
	let mut light = lights[0].path();
	light.push("brightness");
	let lightm = fs::read_to_string(&maxlt)?.lines().next().unwrap()
		.parse::<i32>().unwrap() as f32;

    // Calculate percent
    let mut percent = match args[1].as_str() {
        "inc" => read_light(lightm, &light)? + 5.0,
        "dec" => read_light(lightm, &light)? - 5.0,
        _ => args[1].parse::<i32>().unwrap() as f32,
    };

    if percent <= 0.0 {
        percent = 0.0;
    } else if percent >= 100.0 {
        percent = 100.0;
    }

	// Write Light
//	let percent = args[1].parse::<i32>().unwrap() as f32;
	fs::File::create(&light)?.write_fmt(format_args!("{}",
		(percent * lightm / 100.0).round() as i32))?;

	// Read Light Again
/*	let lightv = fs::read_to_string(&light)?.lines().next().unwrap()
		.parse::<i32>().unwrap() as f32 / lightm;
	println!("Set to: {}%", (lightv * 100.0).round() as u8);*/

	Ok(())
}
