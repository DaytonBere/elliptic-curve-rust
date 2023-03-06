# Elliptic Curve Rust

# Purpose

The purpose of this project is to create a fast and simple way to do Elliptic Curve computations in the Rust language. Once the major features are implemented proper formatting and work will be done to turn this into a Python package.

# To Do:

- [x] Creation of the `EllipticCurve` `struct`
- [x] Creation of the `Point` `struct`
- [x] Integration of `Point` with `EllipticCurve`
- [ ] `EllipticCurve` method for checking if it contains a `Point`. `is_point_in(Point)`
- [ ] `EllipticCurve` `Point` addition method. `add(Point, Point)`
- [ ] `EllipticCurve` `Point` doubling method. `double(Point)`
- [ ] `EllipticCurve` `Point` multiplication method (Double Add Algorithm). `multiply(Point, i128)`
- [ ] Symmetric and Asymetric encription
- [ ] Integration of the origin point `O` as a `Point`

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
