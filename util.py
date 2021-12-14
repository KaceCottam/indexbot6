"""
This file contains some function definitions from APL
that I have grown comfortable using.
"""
from functools import partial as jot
from functools import reduce as reduce
from itertools import accumulate as scan
from operator import *


def atop(f, g):
    """
    This is a function composition operator
    :param f:
    :param g:
    :return:
    """
    return lambda *args: f(g(*args))


def fork(f, g, h):
    """
    This is a function train operator.
    :param f: Evaluate for left side
    :param g: binary merge between left and right
    :param h: Evaluate for right side
    :return: (f g h) x
    """
    return lambda *args: g(f(*args), h(*args))


def ltack(x):
    """
    This is an identity function
    :param x: remaining
    :return: x
    """
    return lambda _: x


def rtack(_):
    """
    Like `ltack`, this is the const function
    :param _: ignored
    :return: next parameter passed in
    """
    return lambda x: x


def pairwise(f):
    """
    Allows us to do a function on pairs from a list.
    :param f: binary function
    :return:
    """
    return lambda l: map(f, l, l[1:])


def flip(f):
    """
    Allows us to flip the operands of a binary operator
    :param f: binary function
    :return: f(b,a)
    """
    return lambda a, b: f(b, a)
