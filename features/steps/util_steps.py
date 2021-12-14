# type: ignore
from behave import *
from util import atop, fork, jot, add, mul
from nose.tools import assert_equals


@when(u"we add {addy:d} then subtract {suby:d} from {x:d}")
def step_impl(context, addy: int, suby: int, x: int):
    context.result = atop(jot(add, -suby), jot(add, addy))(x)


@then(u"we get {result:d}")
def step_impl(context, result: int):
    assert_equals(result, context.result)


@when(u"we both add {addy:d} and subtract {suby:d} from {x:d}, then multiply the result")
def step_impl(context, addy: int, suby: int, x: int):
    context.result = fork(jot(add, addy), mul, jot(add, -suby))(x)
