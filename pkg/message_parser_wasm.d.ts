/* tslint:disable */
/* eslint-disable */
/**
* parses to json AST
* @param {string} s
* @returns {any}
*/
export function parse(s: string): any;

export type ParsedElement =
  | { t: "Text"; c: string }
  | { t: "Tag"; c: string }
  | { t: "Linebreak" }
  | { t: "Bold"; c: ParsedElement[] }
  | { t: "Italics"; c: ParsedElement[] }
  | { t: "StrikeThrough"; c: ParsedElement[] }
  | { t: "InlineCode"; c: { content: string } }
  | { t: "CodeBlock"; c: { language: null | string; content: string } }
  | { t: "EmailAddress"; c: string }
  | { t: "Link"; c: { destination: string } }
  | { t: "LabeledLink"; c: { label: ParsedElement[]; destination: string } };


/**
* Email address struct.
*
* # Examples
* ```
* use email_address_parser::EmailAddress;
*
* assert!(EmailAddress::parse("foo@-bar.com", None).is_none());
* let email = EmailAddress::parse("foo@bar.com", None);
* assert!(email.is_some());
* let email = email.unwrap();
* assert_eq!(email.get_local_part(), "foo");
* assert_eq!(email.get_domain(), "bar.com");
* assert_eq!(format!("{}", email), "foo@bar.com");
* ```
*/
export class EmailAddress {
  free(): void;
/**
* This is a WASM wrapper over EmailAddress::new that panics.
* If you are using this lib from Rust then consider using EmailAddress::new.
*
* # Examples
* ```
* use email_address_parser::EmailAddress;
*
* let email = EmailAddress::_new("foo", "bar.com", None);
* ```
*
* # Panics
*
* This method panics if the local part or domain is invalid.
*
* ```rust,should_panic
* use email_address_parser::EmailAddress;
*
* EmailAddress::_new("foo", "-bar.com", None);
* ```
* @param {string} local_part
* @param {string} domain
* @param {ParsingOptions | undefined} options
*/
  constructor(local_part: string, domain: string, options?: ParsingOptions);
/**
* Parses a given string as an email address.
*
* Accessible from WASM.
*
* Returns `Some(EmailAddress)` if the parsing is successful, else `None`.
* # Examples
* ```
* use email_address_parser::*;
*
* // strict parsing
* let email = EmailAddress::parse("foo@bar.com", None);
* assert!(email.is_some());
* let email = email.unwrap();
* assert_eq!(email.get_local_part(), "foo");
* assert_eq!(email.get_domain(), "bar.com");
*
* // non-strict parsing
* let email = EmailAddress::parse("\u{0d}\u{0a} \u{0d}\u{0a} test@iana.org", Some(ParsingOptions::new(true)));
* assert!(email.is_some());
*
* // parsing invalid address
* let email = EmailAddress::parse("test@-iana.org", Some(ParsingOptions::new(true)));
* assert!(email.is_none());
* let email = EmailAddress::parse("test@-iana.org", Some(ParsingOptions::new(true)));
* assert!(email.is_none());
* let email = EmailAddress::parse("test", Some(ParsingOptions::new(true)));
* assert!(email.is_none());
* let email = EmailAddress::parse("test", Some(ParsingOptions::new(true)));
* assert!(email.is_none());
* ```
* @param {string} input
* @param {ParsingOptions | undefined} options
* @returns {EmailAddress | undefined}
*/
  static parse(input: string, options?: ParsingOptions): EmailAddress | undefined;
/**
* Validates if the given `input` string is an email address or not.
*
* Returns `true` if the `input` is valid, `false` otherwise.
* Unlike the `parse` method, it does not instantiate an `EmailAddress`.
* # Examples
* ```
* use email_address_parser::*;
*
* // strict validation
* assert!(EmailAddress::is_valid("foo@bar.com", None));
*
* // non-strict validation
* assert!(EmailAddress::is_valid("\u{0d}\u{0a} \u{0d}\u{0a} test@iana.org", Some(ParsingOptions::new(true))));
*
* // invalid address
* assert!(!EmailAddress::is_valid("test@-iana.org", Some(ParsingOptions::new(true))));
* assert!(!EmailAddress::is_valid("test@-iana.org", Some(ParsingOptions::new(true))));
* assert!(!EmailAddress::is_valid("test", Some(ParsingOptions::new(true))));
* assert!(!EmailAddress::is_valid("test", Some(ParsingOptions::new(true))));
* ```
* @param {string} input
* @param {ParsingOptions | undefined} options
* @returns {boolean}
*/
  static isValid(input: string, options?: ParsingOptions): boolean;
/**
* Returns the domain of the email address.
*
* Note that if you are using this library from rust, then consider using the `get_domain` method instead.
* This returns a cloned copy of the domain string, instead of a borrowed `&str`, and exists purely for WASM interoperability.
*
* # Examples
* ```
* use email_address_parser::EmailAddress;
*
* let email = EmailAddress::new("foo", "bar.com", None).unwrap();
* assert_eq!(email.domain(), "bar.com");
*
* let email = EmailAddress::parse("foo@bar.com", None).unwrap();
* assert_eq!(email.domain(), "bar.com");
* ```
* @returns {string}
*/
  readonly domain: string;
/**
* Returns the local part of the email address.
*
* Note that if you are using this library from rust, then consider using the `get_local_part` method instead.
* This returns a cloned copy of the local part string, instead of a borrowed `&str`, and exists purely for WASM interoperability.
*
* # Examples
* ```
* use email_address_parser::EmailAddress;
*
* let email = EmailAddress::new("foo", "bar.com", None).unwrap();
* assert_eq!(email.localPart(), "foo");
*
* let email = EmailAddress::parse("foo@bar.com", None).unwrap();
* assert_eq!(email.localPart(), "foo");
* ```
* @returns {string}
*/
  readonly localPart: string;
}
/**
* Options for parsing.
*
* The is only one available option so far `is_lax` which can be set to
* `true` or `false` to  enable/disable obsolete parts parsing.
* The default is `false`.
*/
export class ParsingOptions {
  free(): void;
/**
* @param {boolean} is_lax
*/
  constructor(is_lax: boolean);
/**
* @returns {boolean}
*/
  is_lax: boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly parse: (a: number, b: number) => number;
  readonly __wbg_parsingoptions_free: (a: number) => void;
  readonly __wbg_get_parsingoptions_is_lax: (a: number) => number;
  readonly __wbg_set_parsingoptions_is_lax: (a: number, b: number) => void;
  readonly parsingoptions_new: (a: number) => number;
  readonly __wbg_emailaddress_free: (a: number) => void;
  readonly emailaddress__new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly emailaddress_parse: (a: number, b: number, c: number) => number;
  readonly emailaddress_isValid: (a: number, b: number, c: number) => number;
  readonly emailaddress_localPart: (a: number, b: number) => void;
  readonly emailaddress_domain: (a: number, b: number) => void;
  readonly emailaddress_toString: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
