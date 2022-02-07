I2C Example

This example uses I2C to communicate with a Display and show an Image.

**Requires [SSD1306 Display](https://www.elmorlabs.com/product/0-96-i2c-oled-display-white/) which is not included.**

The Display must be connected to the J2 header. Since the header does not provide 3.3V power it has to be sourced from JP1 or an external source.

By default the display will show the primary image, pressing Button1 will switch to the secondary Image.


Any custom Image can be used by changing the file path in the [build script](https://github.com/juvgrfunex/DERULER-RUST-EXAMPLES/blob/master/i2c_ssd1306/build.rs).
