import pytest

import calculator


def test_add_empty_then_0():
    assert calculator.add() == 0


def test_add_1_and__1_then_0():
    assert calculator.add(1, -1) == 0


def test_add_1_1_then_2():
    assert calculator.add(1, 1) == 2


def test_add__1_and__1_then__2():
    assert calculator.add(-1, -1) == -2


def test_add_3_6_9_then_18():
    assert calculator.add(3, 6, 9) == 18


def test_subtract_empty_then_0():
    assert calculator.subtract() == 0


def test_subtract_1_and__1_then_2():
    assert calculator.subtract(1, -1) == 2


def test_subtract_1_1_then_0():
    assert calculator.subtract(1, 1) == 0


def test_subtract__1_and__1_then_0():
    assert calculator.subtract(-1, -1) == 0


def test_subtract_1_1_1_then__1():
    assert calculator.subtract(1, 1, 1) == -1


def test_multiply_empty_then_0():
    assert calculator.multiply() == 0


def test_multiply_1_and_0_then_0():
    assert calculator.multiply(1, 0) == 0


def test_multiply_2_and__1_then__2():
    assert calculator.multiply(2, -1) == -2


def test_multiply_1_1_then_1():
    assert calculator.multiply(1, 1) == 1


def test_multiply__1_and__1_then_1():
    assert calculator.multiply(-1, -1) == 1


def test_multiply_1_2_3_then_6():
    assert calculator.multiply(1, 2, 3) == 6


def test_divide_empty_then_0():
    assert calculator.divide() == 0


def test_divide_1_and__1_then__1():
    assert calculator.divide(1, -1) == -1


def test_divide_1_0_then_calculator_zero_division_error():
    with pytest.raises(calculator.ZeroDivisionError):
        calculator.divide(1, 0)


def test_divide_0_1_then_0():
    assert calculator.divide(0, 1) == 0


def test_divide__1_and__1_then_1():
    assert calculator.divide(-1, -1) == 1


def test_divide_1_1_1_then_1():
    assert calculator.divide(1, 1, 1) == 1


def test_divide_3_2_1_then_1dot5():
    assert calculator.divide(3, 2, 1) == 1.5
