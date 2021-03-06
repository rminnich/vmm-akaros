/* SPDX-License-Identifier: GPL-2.0-only */

fn main() {
    cc::Build::new().file("c_src/cpuid.c").compile("cpuid");
    cc::Build::new().file("c_src/utils.c").compile("utils");
    cc::Build::new().file("c_src/hlt.s").compile("hlt");
    cc::Build::new()
        .file("c_src/vmnet.m")
        .compile("virtio-vmnet");

    println!("cargo:rustc-link-lib=framework=Hypervisor");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=vmnet");
}
