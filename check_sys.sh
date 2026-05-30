#!/bin/sh
print_file() {
	printf "%s = %s\n" "$1" "$(cat "$1")"
}

files() {
	same="$(cat "${1}0/${2}/${3}")"
	printf "%s = %s\n" "${1}*/${2}/${3}" "$same"
	for file in "${1}"*"/${2}/${3}"; do
		val="$(cat "$file")"
		if [ "$val" != "$same" ]; then
			printf "%s = %s\n" "$file" "$val"
		fi
	done
}

cpu="/sys/devices/system/cpu"
cpu_files() {
	files "${cpu}/cpu" "cpufreq" "$1"
}

print_file /sys/class/power_supply/ACAD/online
echo

echo Platform Profile
print_file /sys/firmware/acpi/platform_profile_choices
print_file /sys/firmware/acpi/platform_profile
echo

echo CPU
cpu_files scaling_driver
cpu_files scaling_available_governors
cpu_files scaling_governor
cpu_files energy_performance_available_preferences
cpu_files energy_performance_preference
echo
print_file "${cpu}/amd_pstate/status"
print_file "${cpu}/cpufreq/boost"
print_file /sys/module/cpufreq/parameters/default_governor
print_file /sys/module/workqueue/parameters/power_efficient
print_file /proc/sys/kernel/nmi_watchdog
echo

echo GPU
print_file /sys/module/amdgpu/parameters/abmlevel
for card in /sys/class/drm/card?/device; do
	echo
	print_file "${card}/power/control"
	if [ "$(cat "${card}/power/runtime_status")" != "suspended" ]; then
		print_file "${card}/power_dpm_state"
		print_file "${card}/power_dpm_force_performance_level"
		if [ -f "${card}/pp_power_profile_mode" ]; then
			echo "${card}/pp_power_profile_mode"
			tail -c +25 "${card}/pp_power_profile_mode" | head -n 1
		fi
	else
		print_file "${card}/power/runtime_status"
	fi
done
echo

echo Audio
snd_hda_intel_power_save="/sys/module/snd_hda_intel/parameters/power_save"
print_file "$snd_hda_intel_power_save"
print_file "${snd_hda_intel_power_save}_controller"
echo

echo ASPM
print_file /sys/module/pcie_aspm/parameters/policy
