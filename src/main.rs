use std::{fs, io, thread::sleep, time::Duration};

fn main() -> io::Result<()> {
    #[cfg(not(target_os = "linux"))]
    compile_error!("Linux only!");

    let mut ac = !plugged_in();
    loop {
        let new_ac = plugged_in();
        if ac != new_ac {
            ac = new_ac;

            if ac {
                println!("Switching to performance.");
                performance()?;
            } else {
                println!("Switching to powersave.");
                powersave()?;
            }
        }
        sleep(Duration::from_secs(10));
    }
}

fn performance() -> io::Result<()> {
    if platform_profile_choices("performance") {
        fs::write("/sys/firmware/acpi/platform_profile", "performance\n")?;
    }

    // CPU
    fs::write("/sys/devices/system/cpu/cpufreq/boost", "1\n")?;
    // TODO: check /sys/devices/system/cpu/cpu*/cpufreq/scaling_available_governors for performance
    // TODO: /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor = performance

    // GPU
    // FIXME: actually check if any of this exists first
    fs::write("/sys/class/drm/card1/device/power/control", "on")?;
    fs::write(
        "/sys/class/drm/card1/device/power_dpm_force_performance_level",
        "manual\n",
    )?;
    fs::write("/sys/class/drm/card1/device/pp_power_profile_mode", "1\n")?;

    // Audio
    fs::write("/sys/module/snd_hda_intel/parameters/power_save", "10\n")?; // maybe 0?
    fs::write(
        "/sys/module/snd_hda_intel/parameters/power_save_controller",
        "Y\n",
    )?; // maybe N?

    Ok(())
}

fn powersave() -> io::Result<()> {
    if platform_profile_choices("low-power") {
        fs::write("/sys/firmware/acpi/platform_profile", "low-power\n")?;
    }

    // CPU
    fs::write("/sys/devices/system/cpu/cpufreq/boost", "0\n")?;
    // TODO: check /sys/devices/system/cpu/cpu*/cpufreq/scaling_available_governors for powersave
    // TODO: /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor = powersave

    // GPU
    // FIXME
    fs::write("/sys/class/drm/card1/device/pp_power_profile_mode", "0\n")?;
    fs::write(
        "/sys/class/drm/card1/device/power_dpm_force_performance_level",
        "auto\n",
    )?;
    fs::write("/sys/class/drm/card1/device/power/control", "auto")?;

    // Audio
    fs::write("/sys/module/snd_hda_intel/parameters/power_save", "1\n")?;
    fs::write(
        "/sys/module/snd_hda_intel/parameters/power_save_controller",
        "Y\n",
    )?;

    Ok(())
}

fn platform_profile_choices(contains: &str) -> bool {
    fs::read_to_string("/sys/firmware/acpi/platform_profile_choices")
        .unwrap_or_default()
        .split_whitespace()
        .any(|s| s == contains)
}

fn plugged_in() -> bool {
    if let Ok(contents) = fs::read_to_string("/sys/class/power_supply/ACAD/online") {
        contents.trim_end() != "0"
    } else {
        true
    }
}
