use num::complex::Complex;

fn mandelbrot_at_point(x: f64, y: f64, iterations: usize) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(x, y);

    for i in 0..iterations {
        if z.norm() > 2.0 {
            return i;
        }

        z = z * z + c;
    }

    return iterations;
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

struct Size {
    width: usize,
    height: usize,
}

impl Size {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

/// @brief Calculates a Mandelbrot set
///
/// @param min Bottom left viewport point
/// @param max Top right viewport point
/// @param size With and height of the image
/// @param iteration Max number of iterations
///
/// @return A 2D image
fn calculate_mandelbrot(min: Point, max: Point, size: Size, iterations: usize) -> Vec<Vec<usize>> {
    let mut columns = Vec::with_capacity(size.width);

    for y in 0..size.height {
        let mut row = Vec::with_capacity(size.height);

        for x in 0..size.width {
            let cx = min.x + (max.x - min.x) * (x as f64 / size.width as f64);
            let cy = min.y + (max.y - min.y) * (y as f64 / size.height as f64);
            let val = mandelbrot_at_point(cx, cy, iterations);

            row.push(val);
        }

        columns.push(row)
    }

    columns
}

/// @brief Associate an ASCII character to numeric values
fn render_mandelbrot(mandelbrot: Vec<Vec<usize>>) {
    for row in mandelbrot {
        // We are going to create a line to print on terminal from a row of values
        let mut line = String::with_capacity(row.len());

        for val in row {
            let char = match val {
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => 'ø',
                11..=30 => '*',
                30..=100 => '+',
                100..=200 => 'x',
                200..=400 => '$',
                400..=700 => '#',
                _ => '%',
            };

            line.push(char);
        }

        println!("{}", line);
    }
}

fn main() {
    // Calculate a mandelbrot set
    let mandelbrot = calculate_mandelbrot(
        Point::new(-2.0, -1.0),
        Point::new(1.0, 1.0),
        Size::new(100, 30),
        1000,
    );

    // Render the Mandelbrot set with ASCII characters
    render_mandelbrot(mandelbrot);
}
