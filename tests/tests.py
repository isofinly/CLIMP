import os
import subprocess


current_file_path = os.path.dirname(os.path.abspath(__file__))
image_path = "../pics/photo.jpg"

def run_rust_cli(args):
    """
    Run the Rust CLI application with the given arguments and return the output.
    """
    try:
        output = subprocess.check_output([current_file_path + '/climp'] + args, stderr=subprocess.STDOUT, text=True)
        return output
    except subprocess.CalledProcessError as e:
        return e.output

def test_blur_command():
    args = [image_path, '-o', './out.png','blur', '-r', '5']
    output = run_rust_cli(args)
    assert "Blurred image saved as" in output
    assert os.path.exists('./out.png')
    print("\x1b[32mBLUR\x1b[0m COMMAND TEST PASSED")
def test_pixelate_command():
    args = [image_path, '-o', './out.png', 'pixelate', '-p', '10']
    output = run_rust_cli(args)
    assert "Pixelated image saved as" in output
    assert os.path.exists('./out.png')
    print("\x1b[32mPIXELATE\x1b[0m COMMAND TEST PASSED")

def test_mirror_command():
    args = [image_path, '-o', './out.png', 'mirror']
    output = run_rust_cli(args)
    assert "Mirrored image saved as" in output
    assert os.path.exists('./out.png')
    print("\x1b[32mMIRROR\x1b[0m COMMAND TEST PASSED")

def test_flip_vertical_command():
    args = [image_path, '-o', './out.png', 'flip_vertical']
    output = run_rust_cli(args)
    assert "Flipped image saved as" in output
    assert os.path.exists('./out.png')
    print("\x1b[32mFLIP\x1b[0m VERTICAL COMMAND TEST PASSED")

def test_rotate_command():
    args = [image_path, '-o', './out.png', 'rotate']
    output = run_rust_cli(args)
    assert "Rotated image saved as" in output
    assert os.path.exists('./out.png')
    print("\x1b[32mROTATE\x1b[0m COMMAND TEST PASSED")

def test_grayscale_command():
    args = [image_path, '-o', './out.png', 'grayscale']
    output = run_rust_cli(args)
    assert "Grayscale image saved as" in output
    assert os.path.exists('./out.png')
    print("\x1b[32mGRAYSCALE\x1b[0m COMMAND TEST PASSED")

def test_monochrome_ugly_command():
    args = [image_path, '-o', './out.png', 'monochrome_ugly']
    output = run_rust_cli(args)
    assert "Monochrome image saved as" in output
    assert os.path.exists('./out.png')
    print("\x1b[32mMONOCHROME\x1b[0m UGLY COMMAND TEST PASSED")

def test_scale_command():
    args = [image_path, '-o', './out.png', 'scale', '-s', '2']
    output = run_rust_cli(args)
    assert "Scaled image saved as" in output
    assert os.path.exists('./out.png')
    print("\x1b[32mSCALE\x1b[0m COMMAND TEST PASSED")


if __name__ == "__main__":
    try:
        test_blur_command()
        test_pixelate_command()
        test_mirror_command()
        test_flip_vertical_command()
        test_rotate_command()
        test_grayscale_command()
        test_monochrome_ugly_command()
        test_scale_command()
        print()
        print("\x1b[32mAll tests passed!\x1b[0m")
    except AssertionError as e:
        print("Test failed:", e)