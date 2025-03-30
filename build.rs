fn main() {
    rm -rf iso_root
	mkdir -p iso_root/boot
	mkdir -p iso_root/boot/limine
	mkdir -p iso_root/EFI/BOOT

	cp kernel/kernel iso_root/boot/
	cp limine.conf iso_root/boot/limine/
	cp limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp limine/BOOTX64.EFI iso_root/EFI/BOOT/
	cp limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	cp userland/build/hello-world.elf iso_root/boot/

	let image_name = "template.iso"

	Command::new("xorriso")
    	.args(&["-as", "mkisofs", "-b", "boot/limine/limine-bios-cd.bin"])
    	.args([ "-no-emul-boot", "-boot-load-size", "4", "-boot-info-table"])
    	.args([ "--efi-boot", "boot/limine/limine-uefi-cd.bin", "-efi-boot-part", "--efi-boot-image"])
    	.args([ "--protective-msdos-label", "iso_root", ])
    	.args([ "-o", &format!("{}.iso", image_name), "2>", "/dev/null"])

     Command::new("./limine/limine")
        .args([
            "bios-install",
            &format!("{}.iso", image_name),
            "2>",
            "/dev/null",
        ]);
}
