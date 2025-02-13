pub fn main() {
    cc::Build::new()
        .file("arm.cmsis-dsp.1.16.2/Source/BasicMathFunctions/BasicMathFunctions.c")
        .compile("cmsis-dsp");
}