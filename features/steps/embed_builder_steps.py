# type: ignore
from behave import *
from nose.tools import assert_equals
from embed_builder import EmbedBuilder


@given(u'an embed builder titled "{title}"')
def step_impl(context, title: str):
    context.embed = EmbedBuilder(title=title)


@given(u'it has an inline field "{name}" with content')
def step_impl(context, name: str):
    value = context.text
    context.embed.add_field(name=name, value=value, inline=True)


@given(u'it has a field "{name}" with content')
def step_impl(context, name: str):
    value = context.text
    context.embed.add_field(name=name, value=value, inline=False)


@when("rendered to a string")
def step_impl(context):
    context.result = str(context.embed)


@then("the string result is")
def step_impl(context):
    result = context.text
    assert_equals(result, context.result)
