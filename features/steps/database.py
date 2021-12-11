# type: ignore
from behave import *
from parse_type import TypeBuilder
from nose.tools import assert_equal, assert_dict_equal, assert_in, assert_is, assert_set_equal
from tinydb import TinyDB
from tinydb.storages import MemoryStorage
import api

# let us parse multiple ids
register_type(id=TypeBuilder.with_zero_or_more(converter=str, pattern=r"\w*"))


@given(u"an empty initial database")
def step_impl(context):
    context.db = TinyDB(storage=MemoryStorage)


@then(u'the users who subscribe to role "{roleid}" in guild "{guildid}" is {result:id}')
def step_impl(context, roleid: str, guildid: str, result: list[str]):
    assert_set_equal(set(api.showUsers(context.db, guildid, roleid)), set(result))


@then(u'the roles that user "{userid}" in guild "{guildid}" has is {result:id}')
def step_impl(context, userid: str, guildid: str, result: list[str]):
    assert_set_equal(set(api.showRolesOfUser(context.db, guildid, userid)), set(result))


@then(u'the roles that guild "{guildid}" has is {result:id}')
def step_impl(context, guildid: str, result: list[str]):
    assert_set_equal(set(api.showRolesOfGuild(context.db, guildid)), set(result))


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


@then('there are no users who subscribe to role "{roleid}" in guild "{guildid}"')
def step_impl(context, roleid: str, guildid: str):
    assert_equal(0, len(api.showUsers(context.db, guildid, roleid)))


@then('there are no roles that user "{userid}" in guild "{guildid}" subscribes to')
def step_impl(context, userid: str, guildid: str):
    assert_equal(0, len(api.showRolesOfUser(context.db, guildid, userid)))


@then('there are no roles in guild "{guildid}"')
def step_impl(context, guildid: str):
    assert_equal(0, len(api.showRolesOfGuild(context.db, guildid)))
