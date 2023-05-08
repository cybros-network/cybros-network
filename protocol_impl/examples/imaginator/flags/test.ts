// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
import { assertEquals } from "https://deno.land/std/testing/asserts.ts";
import { Args, parse, ParseOptions } from "./mod.ts";
import { assertType, IsExact } from "https://deno.land/std/testing/types.ts";

// flag boolean true (default all --args to boolean)
Deno.test("flagBooleanTrue", function () {
  const argv = parse(["moo", "--honk", "cow"], {
    boolean: true,
  });

  assertEquals(argv, {
    honk: true,
    _: ["cow"],
    _1: "moo",
  });

  assertEquals(typeof argv.honk, "boolean");
});

// flag boolean true only affects double hyphen arguments without equals signs
Deno.test("flagBooleanTrueOnlyAffectsDoubleDash", function () {
  const argv = parse(["moo", "--honk", "cow", "-p", "55", "--tacos=good"], {
    boolean: true,
  });

  assertEquals(argv, {
    honk: true,
    tacos: "good",
    p: 55,
    _: ["cow"],
    _1: "moo",
  });

  assertEquals(typeof argv.honk, "boolean");
});

Deno.test("flagBooleanDefaultFalse", function () {
  const argv = parse(["moo"], {
    boolean: ["t", "verbose"],
    default: { verbose: false, t: false },
  });

  assertEquals(argv, {
    verbose: false,
    t: false,
    _: [],
    _1: "moo",
  });

  assertEquals(typeof argv.verbose, "boolean");
  assertEquals(typeof argv.t, "boolean");
});

Deno.test("booleanGroups", function () {
  const argv = parse(["-x", "-z", "one", "two", "three"], {
    boolean: ["x", "y", "z"],
  });

  assertEquals(argv, {
    x: true,
    y: false,
    z: true,
    _: ["one", "two", "three"],
    _1: undefined,
  });

  assertEquals(typeof argv.x, "boolean");
  assertEquals(typeof argv.y, "boolean");
  assertEquals(typeof argv.z, "boolean");
});

Deno.test("booleanAndAliasDefaultsToFalseWithNoArgs", function (): void {
  const argv = parse([], {
    string: ["foo"],
    boolean: ["bar"],
    alias: {
      bar: "b",
    },
  });

  assertEquals(argv, {
    bar: false,
    b: false,
    _: [],
    _1: undefined,
  });
});

Deno.test("booleanAndAliasWithChainableApi", function () {
  const aliased = ["-h", "derp"];
  const regular = ["--herp", "derp"];
  const aliasedArgv = parse(aliased, {
    boolean: "herp",
    alias: { h: "herp" },
  });
  const propertyArgv = parse(regular, {
    boolean: "herp",
    alias: { h: "herp" },
  });
  const expected = {
    herp: true,
    h: true,
    _: ["derp"],
    _1: undefined,
  };

  assertEquals(aliasedArgv, expected);
  assertEquals(propertyArgv, expected);
});

Deno.test("booleanAndAliasWithOptionsHash", function () {
  const aliased = ["-h", "derp"];
  const regular = ["--herp", "derp"];
  const opts = {
    alias: { h: "herp" },
    boolean: "herp",
  } as const;
  const aliasedArgv = parse(aliased, opts);
  const propertyArgv = parse(regular, opts);
  const expected = {
    herp: true,
    h: true,
    _: ["derp"],
    _1: undefined,
  };
  assertEquals(aliasedArgv, expected);
  assertEquals(propertyArgv, expected);
});

Deno.test("booleanAndAliasArrayWithOptionsHash", function () {
  const aliased = ["-h", "derp"];
  const regular = ["--herp", "derp"];
  const alt = ["--harp", "derp"];
  const opts = {
    alias: { h: ["herp", "harp"] },
    boolean: "h",
  } as const;
  const aliasedArgv = parse(aliased, opts);
  const propertyArgv = parse(regular, opts);
  const altPropertyArgv = parse(alt, opts);
  const expected = {
    harp: true,
    herp: true,
    h: true,
    _: ["derp"],
    _1: undefined,
  };
  assertEquals(aliasedArgv, expected);
  assertEquals(propertyArgv, expected);
  assertEquals(altPropertyArgv, expected);
});

Deno.test("booleanAndAliasUsingExplicitTrue", function () {
  const aliased = ["-h", "true"];
  const regular = ["--herp", "true"];
  const opts = {
    alias: { h: "herp" },
    boolean: "h",
  } as const;
  const aliasedArgv = parse(aliased, opts);
  const propertyArgv = parse(regular, opts);
  const expected = {
    herp: true,
    h: true,
    _: [],
    _1: undefined,
  };

  assertEquals(aliasedArgv, expected);
  assertEquals(propertyArgv, expected);
});

// regression, see https://github.com/substack/node-optimist/issues/71
// boolean and --x=true
Deno.test("booleanAndNonBoolean", function () {
  const parsed = parse(["--boool", "--other=true"], {
    boolean: "boool",
  });

  assertEquals(parsed.boool, true);
  assertEquals(parsed.other, "true");

  const parsed2 = parse(["--boool", "--other=false"], {
    boolean: "boool",
  });

  assertEquals(parsed2.boool, true);
  assertEquals(parsed2.other, "false");
});

Deno.test("booleanParsingTrue", function () {
  const parsed = parse(["--boool=true"], {
    default: {
      boool: false,
    },
    boolean: ["boool"],
  });

  assertEquals(parsed.boool, true);
});

Deno.test("booleanParsingFalse", function () {
  const parsed = parse(["--boool=false"], {
    default: {
      boool: true,
    },
    boolean: ["boool"],
  });

  assertEquals(parsed.boool, false);
});

Deno.test("booleanParsingTrueLike", function () {
  const parsed = parse(["-t", "true123"], { boolean: ["t"] });
  assertEquals(parsed.t, true);

  const parsed2 = parse(["-t", "123"], { boolean: ["t"] });
  assertEquals(parsed2.t, true);

  const parsed3 = parse(["-t", "false123"], { boolean: ["t"] });
  assertEquals(parsed3.t, true);
});

Deno.test("booleanNegationAfterBoolean", function () {
  const parsed = parse(["--foo", "--no-foo"], {
    boolean: ["foo"],
    negatable: ["foo"],
  });
  assertEquals(parsed.foo, false);

  const parsed2 = parse(["--foo", "--no-foo", "123"], {
    boolean: ["foo"],
    negatable: ["foo"],
  });
  assertEquals(parsed2.foo, false);
});

Deno.test("booleanAfterBooleanNegation", function () {
  const parsed = parse(["--no-foo", "--foo"], {
    boolean: ["foo"],
    negatable: ["foo"],
  });
  assertEquals(parsed.foo, true);

  const parsed2 = parse(["--no-foo", "--foo", "123"], {
    boolean: ["foo"],
    negatable: ["foo"],
  });
  assertEquals(parsed2.foo, true);
});

Deno.test("latestFlagIsBooleanNegation", function () {
  const parsed = parse(["--no-foo", "--foo", "--no-foo"], {
    boolean: ["foo"],
    negatable: ["foo"],
  });
  assertEquals(parsed.foo, false);

  const parsed2 = parse(["--no-foo", "--foo", "--no-foo", "123"], {
    boolean: ["foo"],
    negatable: ["foo"],
  });
  assertEquals(parsed2.foo, false);
});

Deno.test("latestFlagIsBoolean", function () {
  const parsed = parse(["--foo", "--no-foo", "--foo"], {
    boolean: ["foo"],
    negatable: ["foo"],
  });
  assertEquals(parsed.foo, true);

  const parsed2 = parse(["--foo", "--no-foo", "--foo", "123"], {
    boolean: ["foo"],
    negatable: ["foo"],
  });
  assertEquals(parsed2.foo, true);
});

Deno.test("hyphen", function () {
  assertEquals(parse(["-n", "-"]), { n: "-", _: [], _1: undefined });
  assertEquals(parse(["-"]), { _: ["-"], _1: undefined });
  assertEquals(parse(["-f-"]), { f: "-", _: [], _1: undefined });
  assertEquals(parse(["-b", "-"], { boolean: "b" }), { b: true, _: ["-"], _1: undefined });
  assertEquals(parse(["-s", "-"], { string: "s" }), { s: "-", _: [], _1: undefined });
});

Deno.test("doubleDash", function () {
  assertEquals(parse(["-a", "--", "b"]), { a: true, _: ["b"], _1: undefined });
  assertEquals(parse(["--a", "--", "b"]), { a: true, _: ["b"], _1: undefined });
  assertEquals(parse(["--a", "--", "b"]), { a: true, _: ["b"], _1: undefined });
});

Deno.test("moveArgsAfterDoubleDashIntoOwnArray", function () {
  assertEquals(
    parse(["--name", "John", "before", "--", "after"], { "--": true }),
    {
      name: "John before",
      _: [],
      _1: undefined,
      "--": ["after"],
    },
  );
});

Deno.test("booleanDefaultTrue", function () {
  const argv = parse([], {
    boolean: "sometrue",
    default: { sometrue: true },
  });
  assertEquals(argv.sometrue, true);
});

Deno.test("booleanDefaultFalse", function () {
  const argv = parse([], {
    boolean: "somefalse",
    default: { somefalse: false },
  });
  assertEquals(argv.somefalse, false);
});

Deno.test("booleanDefaultNull", function () {
  const argv = parse([], {
    boolean: "maybe",
    default: { maybe: null },
  });
  assertEquals(argv.maybe, null);
  const argv2 = parse(["--maybe"], {
    boolean: "maybe",
    default: { maybe: null },
  });
  assertEquals(argv2.maybe, true);
});

Deno.test("dottedAlias", function () {
  const argv = parse(["--a.b", "22"], {
    default: { "a.b": 11 },
    alias: { "a.b": "aa.bb" },
  });
  assertEquals(argv.a.b, 22);
  assertEquals(argv.aa.bb, 22);
});

Deno.test("dottedDefault", function () {
  const argv = parse([], { default: { "a.b": 11 }, alias: { "a.b": "aa.bb" } });
  assertEquals(argv.a.b, 11);
  assertEquals(argv.aa.bb, 11);
});

Deno.test("dottedDefaultWithNoAlias", function () {
  const argv = parse([], { default: { "a.b": 11 } });
  assertEquals(argv.a.b, 11);
});

Deno.test("short", function () {
  const argv = parse(["-b=123"]);
  assertEquals(argv, { b: 123, _: [], _1: undefined });
});

Deno.test("multiShort", function () {
  const argv = parse(["-a=whatever", "-b=robots"]);
  assertEquals(argv, { a: "whatever", b: "robots", _: [], _1: undefined });
});

Deno.test("longOpts", function () {
  assertEquals(parse(["--bool"]), { bool: true, _: [], _1: undefined });
  assertEquals(parse(["--pow", "xixxle"]), { pow: "xixxle", _: [], _1: undefined });
  assertEquals(parse(["--pow=xixxle"]), { pow: "xixxle", _: [], _1: undefined });
  assertEquals(parse(["--host", "localhost", "--port", "555"]), {
    host: "localhost",
    port: 555,
    _: [],
    _1: undefined,
  });
  assertEquals(parse(["--host=localhost", "--port=555"]), {
    host: "localhost",
    port: 555,
    _: [],
    _1: undefined,
  });
});

Deno.test("nums", function () {
  const argv = parse([
    "-x",
    "1234",
    "-y",
    "5.67",
    "-z",
    "1e7",
    "-w",
    "10f",
    "--hex=0xdeadbeef",
    "789",
  ]);
  assertEquals(argv, {
    x: 1234,
    y: 5.67,
    z: 1e7,
    w: "10f",
    hex: 0xdeadbeef,
    _: [789],
    _1: undefined,
  });
  assertEquals(typeof argv.x, "number");
  assertEquals(typeof argv.y, "number");
  assertEquals(typeof argv.z, "number");
  assertEquals(typeof argv.w, "string");
  assertEquals(typeof argv.hex, "number");
  assertEquals(typeof argv._[0], "number");
});

Deno.test("alreadyNumber", function () {
  const argv = parse(["-x=1234", "789"]);
  assertEquals(argv, { x: 1234, _: [789], _1: undefined });
  assertEquals(typeof argv.x, "number");
  assertEquals(typeof argv._[0], "number");
});

Deno.test("parseArgs", function () {
  assertEquals(parse(["--no-moo"]), { "no-moo": true, _: [], _1: undefined });
  assertEquals(parse(["-v", "a", "-v", "b", "-v", "c"]), {
    v: "c",
    _: [],
    _1: undefined,
  });
});

Deno.test("comprehensive", function () {
  assertEquals(
    parse([
      "--name=meowmers",
      "bare",
      "-cats",
      "woo",
      "-h",
      "awesome",
      "--multi=quux",
      "--key",
      "value",
      "-b",
      "--bool",
      "--no-meep",
      "--multi=baz",
      "-f=abc=def",
      "--foo=---=\\n--+34-=/=",
      "-e==",
      "--",
      "--not-a-flag",
      "eek",
    ]),
    {
      c: true,
      a: true,
      t: true,
      e: "=",
      f: "abc=def",
      foo: "---=\\n--+34-=/=",
      s: "woo",
      h: "awesome",
      b: true,
      bool: true,
      key: "value",
      multi: "baz",
      "no-meep": true,
      name: "meowmers",
      _: ["bare", "--not-a-flag", "eek"],
      _1: undefined,
    },
  );
});

Deno.test("flagBoolean", function () {
  const argv = parse(["-t", "moo"], { boolean: "t" });
  assertEquals(argv, { t: true, _: ["moo"], _1: undefined });
  assertEquals(typeof argv.t, "boolean");
});

Deno.test("flagBooleanValue", function () {
  const argv = parse(["--verbose=false", "moo", "-t", "true"], {
    boolean: ["t", "verbose"],
    default: { verbose: true },
  });

  assertEquals(argv, {
    verbose: false,
    t: true,
    _: ["moo"],
    _1: undefined,
  });

  assertEquals(typeof argv.verbose, "boolean");
  assertEquals(typeof argv.t, "boolean");
});

Deno.test("newlinesInParams", function () {
  const args = parse(["-s", "X\nX"]);
  assertEquals(args, { _: [], s: "X\nX", _1: undefined });

  // reproduce in bash:
  // VALUE="new
  // line"
  // deno program.js --s="$VALUE"
  const args2 = parse(["--s=X\nX"]);
  assertEquals(args2, { _: [], _1: undefined, s: "X\nX" });
});

Deno.test("strings", function () {
  const s = parse(["-s", "0001234"], { string: "s" }).s;
  assertEquals(s, "0001234");
  assertEquals(typeof s, "string");

  const x = parse(["-x", "56"], { string: "x" }).x;
  assertEquals(x, "56");
  assertEquals(typeof x, "string");
});

Deno.test("stringArgs", function () {
  const s = parse(["  ", "  "], { string: "_" })._1;
  assertEquals(s, "     ");
});

Deno.test("emptyStrings", function () {
  const s = parse(["-s"], { string: "s" }).s;
  assertEquals(s, "");
  assertEquals(typeof s, "string");

  const str = parse(["--str"], { string: "str" }).str;
  assertEquals(str, "");
  assertEquals(typeof str, "string");

  const letters = parse(["-art"], {
    string: ["a", "t"],
  });

  assertEquals(letters.a, "");
  assertEquals(letters.r, true);
  assertEquals(letters.t, "");
});

Deno.test("stringAndAlias", function () {
  const x = parse(["--str", "000123"], {
    string: "s",
    alias: { s: "str" },
  });

  assertEquals(x.str, "000123");
  assertEquals(typeof x.str, "string");
  assertEquals(x.s, "000123");
  assertEquals(typeof x.s, "string");

  const y = parse(["-s", "000123"], {
    string: "str",
    alias: { str: "s" },
  });

  assertEquals(y.str, "000123");
  assertEquals(typeof y.str, "string");
  assertEquals(y.s, "000123");
  assertEquals(typeof y.s, "string");
});

Deno.test("slashBreak", function () {
  assertEquals(parse(["-I/foo/bar/baz"]), { I: "/foo/bar/baz", _: [], _1: undefined });
  assertEquals(parse(["-xyz/foo/bar/baz"]), {
    x: true,
    y: true,
    z: "/foo/bar/baz",
    _: [],
    _1: undefined,
  });
});

Deno.test("alias", function () {
  const argv = parse(["-f", "11", "--zoom", "55"], {
    alias: { z: "zoom" },
  });
  assertEquals(argv.zoom, 55);
  assertEquals(argv.z, argv.zoom);
  assertEquals(argv.f, 11);
});

Deno.test("multiAlias", function () {
  const argv = parse(["-f", "11", "--zoom", "55"], {
    alias: { z: ["zm", "zoom"] },
  });
  assertEquals(argv.zoom, 55);
  assertEquals(argv.z, argv.zoom);
  assertEquals(argv.z, argv.zm);
  assertEquals(argv.f, 11);
});

Deno.test("nestedDottedObjects", function () {
  const argv = parse([
    "--foo.bar",
    "3",
    "--foo.baz",
    "4",
    "--foo.quux.quibble",
    "5",
    "--foo.quux.oO",
    "--beep.boop",
  ]);

  assertEquals(argv.foo, {
    bar: 3,
    baz: 4,
    quux: {
      quibble: 5,
      oO: true,
    },
  });
  assertEquals(argv.beep, { boop: true });
});

Deno.test("flagBuiltinProperty", function () {
  const argv = parse(["--toString", "--valueOf", "foo"]);
  assertEquals(argv, { toString: true, valueOf: "foo", _: [], _1: undefined });
  assertEquals(typeof argv.toString, "boolean");
  assertEquals(typeof argv.valueOf, "string");
});

Deno.test("numericShortArgs", function () {
  assertEquals(parse(["-n123"]), { n: 123, _: [], _1: undefined });
  assertEquals(parse(["-123", "456"]), { 1: true, 2: true, 3: 456, _: [], _1: undefined });
});

Deno.test("short", function () {
  assertEquals(parse(["-b"]), { b: true, _: [], _1: undefined });
  assertEquals(parse(["foo", "bar", "baz"]), { _: [], _1: "foo bar baz" });
  assertEquals(parse(["-cats"]), { c: true, a: true, t: true, s: true, _: [], _1: undefined });
  assertEquals(parse(["-cats", "meow"]), {
    c: true,
    a: true,
    t: true,
    s: "meow",
    _: [],
    _1: undefined,
  });
  assertEquals(parse(["-h", "localhost"]), { h: "localhost", _: [], _1: undefined });
  assertEquals(parse(["-h", "localhost", "-p", "555"]), {
    h: "localhost",
    p: 555,
    _: [],
    _1: undefined,
  });
});

Deno.test("mixedShortBoolAndCapture", function () {
  assertEquals(parse(["-h", "localhost", "-fp=555", "script.js"]), {
    f: true,
    p: 555,
    h: "localhost",
    _: ["script.js"],
    _1: undefined,
  });
});

Deno.test("shortAndLong", function () {
  assertEquals(parse(["-h", "localhost", "-fp", "555", "-b"]), {
    f: true,
    p: 555,
    h: "localhost",
    b: true,
    _: [],
    _1: undefined,
  });
});

// stops parsing on the first non-option when stopEarly is set
Deno.test("stopParsing", function () {
  const argv = parse(["--aaa=bbb", "ccc", "--ddd"], {
    stopEarly: true,
  });

  assertEquals(argv, {
    aaa: "bbb",
    _: ["ccc", "--ddd"],
    _1: undefined,
  });
});

Deno.test("booleanAndAliasIsNotUnknown", function () {
  const unknown: unknown[] = [];
  function unknownFn(arg: string, k?: string, v?: unknown): boolean {
    unknown.push({ arg, k, v });
    return false;
  }
  const aliased = ["-h", "true", "--derp", "true"];
  const regular = ["--herp", "true", "-d", "false"];
  const opts = {
    alias: { h: "herp" },
    boolean: "h",
    unknown: unknownFn,
  };
  parse(aliased, opts);
  parse(regular, opts);

  assertEquals(unknown, [
    { arg: "--derp", k: "derp", v: "true" },
    { arg: "-d", k: "d", v: "false" },
  ]);
});

Deno.test(
  "flagBooleanTrueAnyDoubleHyphenArgumentIsNotUnknown",
  function () {
    const unknown: unknown[] = [];
    function unknownFn(arg: string, k?: string, v?: unknown): boolean {
      unknown.push({ arg, k, v });
      return false;
    }
    const argv = parse(["--honk", "--tacos=good", "cow", "-p", "55"], {
      boolean: true,
      unknown: unknownFn,
    });
    assertEquals(unknown, [
      { arg: "--tacos=good", k: "tacos", v: "good" },
      { arg: "cow", k: undefined, v: undefined },
      { arg: "-p", k: "p", v: "55" },
    ]);
    assertEquals(argv, {
      honk: true,
      _: [],
      _1: undefined,
    });
  },
);

Deno.test("stringAndAliasIsNotUnknown", function () {
  const unknown: unknown[] = [];
  function unknownFn(arg: string, k?: string, v?: unknown): boolean {
    unknown.push({ arg, k, v });
    return false;
  }
  const aliased = ["-h", "hello", "--derp", "goodbye"];
  const regular = ["--herp", "hello", "-d", "moon"];
  const opts = {
    alias: { h: "herp" },
    string: "h",
    unknown: unknownFn,
  };
  parse(aliased, opts);
  parse(regular, opts);

  assertEquals(unknown, [
    { arg: "--derp", k: "derp", v: "goodbye" },
    { arg: "-d", k: "d", v: "moon" },
  ]);
});

Deno.test("defaultAndAliasIsNotUnknown", function () {
  const unknown: unknown[] = [];
  function unknownFn(arg: string, k?: string, v?: unknown): boolean {
    unknown.push({ arg, k, v });
    return false;
  }
  const aliased = ["-h", "hello"];
  const regular = ["--herp", "hello"];
  const opts = {
    default: { h: "bar" },
    alias: { h: "herp" },
    unknown: unknownFn,
  };
  parse(aliased, opts);
  parse(regular, opts);

  assertEquals(unknown, []);
});

Deno.test("valueFollowingDoubleHyphenIsNotUnknown", function () {
  const unknown: unknown[] = [];
  function unknownFn(arg: string, k?: string, v?: unknown): boolean {
    unknown.push({ arg, k, v });
    return false;
  }
  const aliased = ["--bad", "--", "good", "arg"];
  const opts = {
    "--": true,
    unknown: unknownFn,
  };
  const argv = parse(aliased, opts);

  assertEquals(unknown, [{ arg: "--bad", k: "bad", v: true }]);
  assertEquals(argv, {
    "--": ["good", "arg"],
    _: [],
    _1: undefined,
  });
});

Deno.test("whitespaceShouldBeWhitespace", function () {
  assertEquals(parse(["-x", "\t"]).x, "\t");
});

Deno.test("collectArgsDefaultBehaviour", function () {
  const argv = parse([
    "--foo",
    "bar",
    "--foo",
    "baz",
    "--beep",
    "boop",
    "--bool",
    "--bool",
  ]);

  assertEquals(argv, {
    foo: "baz",
    beep: "boop",
    bool: true,
    _: [],
    _1: undefined,
  });
});

Deno.test("collectUnknownArgs", function () {
  const argv = parse([
    "--foo",
    "bar",
    "--foo",
    "baz",
    "--beep",
    "boop",
    "--bib",
    "--bib",
    "--bab",
    "--bab",
  ], {
    collect: ["beep", "bib"],
  });

  assertEquals(argv, {
    foo: "baz",
    beep: ["boop"],
    bib: [true, true],
    bab: true,
    _: [],
    _1: undefined,
  });
});

Deno.test("collectArgs", function () {
  const argv = parse([
    "--bool",
    "--bool",
    "--boolArr",
    "--str",
    "foo",
    "--strArr",
    "beep",
    "--unknown",
    "--unknownArr",
  ], {
    boolean: ["bool", "boolArr"],
    string: ["str", "strArr"],
    collect: ["boolArr", "strArr", "unknownArr"],
    alias: {
      bool: "b",
      strArr: "S",
      boolArr: "B",
    },
  });

  assertEquals(argv, {
    bool: true,
    b: true,
    boolArr: [true],
    B: [true],
    str: "foo",
    strArr: ["beep"],
    S: ["beep"],
    unknown: true,
    unknownArr: [true],
    _: [],
    _1: undefined,
  });
});

Deno.test("collectNegateableArgs", function () {
  const argv = parse([
    "--foo",
    "123",
    "-f",
    "456",
    "--no-foo",
  ], {
    string: ["foo"],
    collect: ["foo"],
    negatable: ["foo"],
    alias: {
      foo: "f",
    },
  });

  assertEquals(argv, {
    foo: false,
    f: false,
    _: [],
    _1: undefined,
  });
});

/** ---------------------- TYPE TESTS ---------------------- */

Deno.test("typesOfDefaultOptions", function () {
  const argv = parse([]);
  assertType<
    IsExact<
      typeof argv,
      // deno-lint-ignore no-explicit-any
      & { [x: string]: any }
      & {
        _: Array<string | number>;
      }
      & {
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAllBooleanDisabled", function () {
  const argv = parse([], {
    boolean: false,
  });
  assertType<
    IsExact<
      typeof argv,
      // deno-lint-ignore no-explicit-any
      & { [x: string]: any }
      & {
        _: Array<string | number>;
      }
      & {
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAllBooleanDisabledWithDefaults", function () {
  const argv = parse([], {
    boolean: false,
    default: {
      bar: 123,
    },
  });
  assertType<
    IsExact<
      typeof argv,
      // deno-lint-ignore no-explicit-any
      & { [x: string]: any }
      & {
        _: Array<string | number>;
      }
      & {
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAllBooleanDisabledAndStringArgs", function () {
  const argv = parse([], {
    boolean: false,
    string: ["foo"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo?: string | undefined;
        _: Array<string | number>;
      }
      & {
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAllBooleanDisabledAndStringArgsWithDefaults", function () {
  const argv = parse([], {
    boolean: false,
    string: ["foo"],
    default: {
      foo: 123,
      bar: false,
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: string | number;
        bar: unknown;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAllBoolean", function () {
  const argv = parse([], {
    boolean: true,
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        _: Array<string | number>;
      }
      & {
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAllBooleanWithDefaults", function () {
  const argv = parse([], {
    boolean: true,
    default: {
      foo: "123",
      bar: 123,
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: unknown;
        bar: unknown;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAllBooleanAndStringArgs", function () {
  const argv = parse([], {
    boolean: true,
    string: ["foo", "bar", "foo-bar"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo?: string | undefined;
        bar?: string | undefined;
        "foo-bar"?: string | undefined;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAllBooleanAndStringArgsWithDefaults", function () {
  const argv = parse([], {
    boolean: true,
    string: ["foo", "bar", "foo-bar"],
    default: {
      bar: 123,
      baz: new Date(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo?: string | undefined;
        bar: string | number;
        baz: unknown;
        "foo-bar"?: string | undefined;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfBooleanArgs", function () {
  const argv = parse([], {
    boolean: ["foo", "bar", "foo-bar"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: boolean;
        bar: boolean;
        "foo-bar": boolean;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfBooleanArgsWithDefaults", function () {
  const argv = parse([], {
    boolean: ["foo", "bar", "foo-bar"],
    default: {
      bar: 123,
      baz: "123",
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: boolean;
        bar: number | boolean;
        baz: unknown;
        "foo-bar": boolean;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfStringArgs", function () {
  const argv = parse([], {
    string: ["foo", "bar", "foo-bar"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo?: string | undefined;
        bar?: string | undefined;
        "foo-bar"?: string | undefined;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfStringArgsWithDefaults", function () {
  const argv = parse([], {
    string: ["foo", "bar", "foo-bar"],
    default: {
      bar: true,
      baz: 123,
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo?: string | undefined;
        bar: string | boolean;
        baz: unknown;
        "foo-bar"?: string | undefined;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfBooleanAndStringArgs", function () {
  const argv = parse([], {
    boolean: ["foo", "bar", "foo-bar"],
    string: ["beep", "boop", "beep-boop"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        beep?: string | undefined;
        boop?: string | undefined;
        "beep-boop"?: string | undefined;
        foo: boolean;
        bar: boolean;
        "foo-bar": boolean;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfBooleanAndStringArgsWithDefaults", function () {
  const argv = parse([], {
    boolean: ["foo", "bar", "foo-bar"],
    string: ["beep", "boop", "beep-boop"],
    default: {
      bar: 123,
      baz: new Error(),
      beep: new Date(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: boolean;
        boop?: string | undefined;
        "beep-boop"?: string | undefined;
        bar: number | boolean;
        baz: unknown;
        beep: string | Date;
        "foo-bar": boolean;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

/** ------------------------ DOTTED OPTIONS ------------------------ */

Deno.test("typesOfDottedBooleanArgs", function () {
  const argv = parse([], {
    boolean: ["blubb", "foo.bar", "foo.baz.biz", "foo.baz.buz"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        blubb: boolean;
        foo: {
          bar: boolean;
          baz: {
            biz: boolean;
            buz: boolean;
          };
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfDottedBooleanArgsWithDefaults", function () {
  const argv = parse([], {
    boolean: ["blubb", "foo.bar", "foo.baz.biz", "foo.baz.buz"],
    default: {
      blubb: "123",
      foo: {
        bar: 123,
        baz: {
          biz: new Date(),
        },
      },
      bla: new Date(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        blubb: boolean | string;
        foo: {
          bar: boolean | number;
          baz: {
            biz: boolean | Date;
            buz: boolean;
          };
        };
        bla: unknown;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfDottedStringArgs", function () {
  const argv = parse([], {
    string: ["blubb", "foo.bar", "foo.baz.biz", "foo.baz.buz"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        blubb?: string | undefined;
        foo?: {
          bar?: string | undefined;
          baz?: {
            biz?: string | undefined;
            buz?: string | undefined;
          };
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfDottedStringArgsWithDefaults", function () {
  const argv = parse([], {
    string: ["blubb", "foo.bar", "foo.baz.biz", "foo.baz.buz"],
    default: {
      blubb: true,
      foo: {
        bar: 123,
        baz: {
          biz: new Date(),
        },
      },
      bla: new Date(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        blubb: string | boolean;
        foo: {
          bar: string | number;
          baz: {
            biz: string | Date;
            buz?: string | undefined;
          };
        };
        bla: unknown;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfDottedStringAndBooleanArgs", function () {
  const argv = parse([], {
    boolean: ["blubb", "foo.bar", "foo.baz.biz", "beep.bib.bub"],
    string: ["bla", "beep.boop", "beep.bib.bab", "foo.baz.buz"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        blubb: boolean;
        foo: {
          bar: boolean;
          baz: {
            biz: boolean;
            buz?: string | undefined;
          };
        };
        bla?: string | undefined;
        beep: {
          boop?: string | undefined;
          bib: {
            bab?: string | undefined;
            bub: boolean;
          };
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfDottedStringAndBooleanArgsWithDefaults", function () {
  const argv = parse([], {
    boolean: ["blubb", "foo.bar", "foo.baz.biz", "beep.bib.bub"],
    string: ["beep.boop", "beep.bib.bab", "foo.baz.buz"],
    default: {
      blubb: true,
      foo: {
        bar: 123,
        baz: {
          biz: new Date(),
        },
      },
      beep: {
        boop: true,
        bib: {
          bab: new Date(),
        },
      },
      bla: new Date(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        bla: unknown;
        blubb: boolean;
        foo: {
          bar: boolean | number;
          baz: {
            biz: boolean | Date;
            buz?: string | undefined;
          };
        };
        beep: {
          boop: string | boolean;
          bib: {
            bab: string | Date;
            bub: boolean;
          };
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfDottedStringAndBooleanArgsWithFlattedDefaults", function () {
  const argv = parse([], {
    boolean: ["blubb", "foo.bar", "foo.baz.biz", "beep.bib.bub"],
    string: ["beep.boop", "beep.bib.bab", "foo.baz.buz"],
    default: {
      bla: new Date(),
      blubb: true,
      "foo.bar": 123,
      "foo.baz.biz": new Date(),
      "beep.boop": true,
      "beep.bib.bab": new Date(),
      "mee.moo": true,
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        bla: unknown;
        blubb: boolean;
        mee: unknown;
        foo: {
          bar: boolean | number;
          baz: {
            biz: boolean | Date;
            buz?: string | undefined;
          };
        };
        beep: {
          boop: string | boolean;
          bib: {
            bab: string | Date;
            bub: boolean;
          };
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfDottedArgsWithUnionDefaults", function () {
  const argv = parse([], {
    string: ["foo.bar.baz"],
    boolean: ["beep.boop.bab"],
    default: {
      "foo": 1,
      "beep": new Date(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: number | {
          bar?: {
            baz?: string | undefined;
          } | undefined;
        };
        beep: Date | {
          boop: {
            bab: boolean;
          };
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfDottedArgsWithNestedUnionDefaults", function () {
  const argv = parse([], {
    string: ["foo.bar.baz"],
    boolean: ["beep.boop.bab"],
    default: {
      "foo.bar": 1,
      "beep.boop": new Date(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: {
          bar: number | {
            baz?: string | undefined;
          };
        };
        beep: {
          boop: Date | {
            bab: boolean;
          };
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfArgsWithDottedDefaults", function () {
  const argv = parse([], {
    string: ["foo"],
    default: {
      "foo.bar": 1,
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: string | {
          bar: number;
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

/** ------------------------ COLLECT OPTION -------------------------- */

Deno.test("typesOfCollectUnknownArgs", function () {
  const argv = parse([], {
    collect: ["foo", "bar.baz"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: Array<unknown>;
        bar: {
          baz: Array<unknown>;
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfCollectArgs", function () {
  const argv = parse([], {
    boolean: ["foo", "dotted.beep"],
    string: ["bar", "dotted.boop"],
    collect: ["foo", "dotted.boop"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        bar?: string | undefined;
        dotted: {
          boop: Array<string>;
          beep: boolean;
        };
        foo: Array<boolean>;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfCollectArgsWithDefaults", function () {
  const argv = parse([], {
    boolean: ["foo", "dotted.beep"],
    string: ["bar", "dotted.boop"],
    collect: ["foo", "dotted.boop"],
    default: {
      bar: 123,
      dotted: {
        beep: new Date(),
        boop: /.*/,
      },
      foo: new TextDecoder(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        bar: number | string;
        foo: TextDecoder | Array<boolean>;
        dotted: {
          beep: boolean | Date;
          boop: RegExp | Array<string>;
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfCollectArgsWithSingleArgs", function () {
  const argv = parse([], {
    boolean: ["foo"],
    collect: ["foo"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: Array<boolean>;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfCollectArgsWithEmptyTypeArray", function () {
  const argv = parse([], {
    boolean: [],
    collect: ["foo"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: Array<unknown>;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfCollectArgsWithUnknownArgs", function () {
  const argv = parse([], {
    boolean: ["bar"],
    collect: ["foo"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        bar: boolean;
        foo: Array<unknown>;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfCollectArgsWithKnownAndUnknownArgs", function () {
  const argv = parse([], {
    boolean: ["foo"],
    collect: ["foo", "bar"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: Array<boolean>;
        bar: Array<unknown>;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

/** -------------------------- NEGATABLE OPTIONS --------------------------- */

Deno.test("typesOfNegatableArgs", function () {
  const argv = parse([], {
    boolean: ["foo", "bar", "dotted.tick", "dotted.tock"],
    string: ["beep", "boop", "dotted.zig", "dotted.zag"],
    negatable: ["bar", "boop", "dotted.tick", "dotted.zig"],
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        beep?: string | undefined;
        boop?: string | false | undefined;
        dotted: {
          zig?: string | false | undefined;
          zag?: string | undefined;
          tick: boolean;
          tock: boolean;
        };
        foo: boolean;
        bar: boolean;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfCollectAllArgsWithDefaults", function () {
  const argv = parse([], {
    boolean: ["foo", "bar", "dotted.tick", "dotted.tock"],
    string: ["beep", "boop", "dotted.zig", "dotted.zag"],
    negatable: ["bar", "boop", "dotted.tick", "dotted.zig"],
    default: {
      bar: 123,
      boop: new TextDecoder(),
      dotted: {
        tick: new Date(),
        zig: /.*/,
      },
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo: boolean;
        beep?: string | undefined;
        bar: number | boolean;
        boop: string | false | TextDecoder;
        dotted: {
          zag?: string | undefined;
          tock: boolean;
          tick: boolean | Date;
          zig: string | false | RegExp;
        };
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

/** ----------------------------- ALIAS OPTION ----------------------------- */

Deno.test("typesOfAliasArgs", function () {
  const argv = parse([], {
    boolean: ["foo"],
    string: ["beep"],
    alias: {
      foo: ["bar", "baz"],
      beep: "boop",
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        beep?: string | undefined;
        boop?: string | undefined;
        foo: boolean;
        bar: boolean;
        baz: boolean;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfAliasArgsWithOptions", function () {
  const argv = parse([], {
    boolean: ["foo", "biz"],
    string: ["beep", "bib"],
    negatable: ["foo", "beep"],
    alias: {
      foo: "bar",
      beep: "boop",
      biz: "baz",
    },
    default: {
      foo: 1,
      beep: new Date(),
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        baz: boolean;
        biz: boolean;
        bib?: string | undefined;
        foo: number | boolean;
        bar: number | boolean;
        beep: string | false | Date;
        boop: string | false | Date;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

/** ----------------------- OTHER TYPE TESTS ------------------------ */

Deno.test("typesOfDoubleDashOption", function () {
  const argv = parse([], {
    boolean: true,
    string: ["foo"],
    "--": true,
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo?: string | undefined;
        _: Array<string | number>;
        _1?: string;
        "--": Array<string>;
      }
    >
  >(true);
});

Deno.test("typesOfNullishDefaults", function () {
  const argv = parse([], {
    boolean: true,
    string: ["foo", "bar", "baz"],
    default: {
      bar: undefined,
      baz: null,
    },
  });
  assertType<
    IsExact<
      typeof argv,
      & { [x: string]: unknown }
      & {
        foo?: string | undefined;
        bar: string | undefined;
        baz: string | null;
        _: Array<string | number>;
        _1?: string;
      }
    >
  >(true);
});

Deno.test("typesOfParseGenerics", function () {
  const argv = parse<{ foo?: number } & { bar: string }, true>([]);
  assertType<
    IsExact<
      typeof argv,
      {
        foo?: number | undefined;
        bar: string;
        _: Array<string | number>;
        _1?: string;
        "--": Array<string>;
      }
    >
  >(true);
});

Deno.test("typesOfArgsGenerics", function () {
  type ArgsResult = Args<{ foo?: number } & { bar: string }, true>;
  assertType<
    IsExact<
      ArgsResult,
      {
        foo?: number | undefined;
        bar: string;
        _: Array<string | number>;
        _1?: string;
        "--": Array<string>;
      }
    >
  >(true);
});

Deno.test("typesOfParseOptionsGenerics", function () {
  type Opts = ParseOptions<"foo", "bar" | "baz">;
  assertType<
    IsExact<
      Pick<Opts, "string">,
      { string?: "bar" | "baz" | ReadonlyArray<"bar" | "baz"> | undefined }
    >
  >(true);

  assertType<
    IsExact<
      Pick<Opts, "boolean">,
      { boolean?: "foo" | ReadonlyArray<"foo"> | undefined }
    >
  >(true);

  assertType<
    IsExact<
      Pick<Opts, "default">,
      {
        default?: {
          [x: string]: unknown;
          bar?: unknown;
          baz?: unknown;
          foo?: unknown;
        } | undefined;
      }
    >
  >(true);
});

Deno.test("typesOfParseOptionsGenericDefaults", function () {
  const opts: ParseOptions = {
    boolean: ["foo"],
    string: ["bar"],
  };

  const args = parse([], opts);

  assertType<
    IsExact<
      typeof args,
      {
        // deno-lint-ignore no-explicit-any
        [x: string]: any;
        _: (string | number)[];
        _1?: string;
        "--"?: string[] | undefined;
      }
    >
  >(true);
});
