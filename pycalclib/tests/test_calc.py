# -*- coding: utf-8 -*-

from pycalclib_rs import Calc


def test_add():
    calc = Calc()
    calc.add(1.0)
    assert 1.0 == calc.result


def test_sub():
    calc = Calc()
    calc.sub(1.5)
    assert -1.5 == calc.result
