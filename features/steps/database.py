# type: ignore
from behave import *
from nose.tools import assert_equal, assert_dict_equal, assert_in, assert_is

import api


@given(u"an empty initial database")
def step_impl(context):
    context.db = api.initDB(":memory:")


@then(u'the number of users who subscribe to role "{roleid}" in guild "{guildid}" is {result:d}')
def step_impl(context, roleid: str, guildid: str, result: int):
    assert_equal(len(api.showUsers(context.db, guildid, roleid)), result)


@then(u'the number of roles that user "{userid}" in guild "{guildid}" has is {result:d}')
def step_impl(context, userid: str, guildid: str, result: int):
    assert_equal(len(api.showRolesOfUser(context.db, guildid, userid)), result)


@then(u'the number of roles that guild "{guildid}" has is {result:d}')
def step_impl(context, guildid: str, result: int):
    assert_equal(len(api.showRolesOfGuild(context.db, guildid)), result)


@when(u'inserting user "{userid}" in guild "{guildid}" to role "{roleid}"')
def step_impl(context, userid: str, guildid: str, roleid: str):
    api.addUserToRole(context.db, guildid, roleid, userid)


@given(u'it contains user "{userid}" in role "{roleid}" in guild "{guildid}"')
def step_impl(context, userid: str, roleid: str, guildid: str):
    api.addUserToRole(context.db, guildid, roleid, userid)


@when('removing role "{roleid}" from guild "{guildid}"')
def step_impl(context, roleid: str, guildid: str):
    context.result = api.removeRole(context.db, guildid, roleid)


@when('removing user "{userid}" in guild "{guildid}" from role "{roleid}"')
def step_impl(context, userid: str, guildid: str, roleid: str):
    context.result = api.removeUserFromRole(context.db, guildid, roleid, userid)


@then('the result is user "{userid}" in guild "{guildid}" from role "{roleid}"')
def step_impl(context, userid: str, guildid: str, roleid: str):
    assert_dict_equal(
        dict(guildid=guildid, roleid=roleid, userid=userid), context.result
    )


@then('the result contains the user "{userid}"')
def step_impl(context, userid: str):
    assert_in(userid, context.result)


@then('the result contains user "{userid}" in guild "{guildid}" from role "{roleid}"')
def step_impl(context, userid: str, guildid: str, roleid: str):
    assert_in(dict(guildid=guildid, roleid=roleid, userid=userid), context.result)


@when('removing guild "{guildid}"')
def step_impl(context, guildid: str):
    context.result = api.removeGuild(context.db, guildid)


@then("the result is none")
def step_impl(context):
    assert_is(context.result, None)


@when('removing user "{userid}" in guild "{guildid}"')
def step_impl(context, userid: str, guildid: str):
    context.result = api.removeUser(context.db, guildid, userid)


@then('the result contains the role "{roleid}"')
def step_impl(context, roleid: str):
    assert_in(roleid, context.result)
