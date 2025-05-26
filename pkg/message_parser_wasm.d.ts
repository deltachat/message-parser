/* tslint:disable */
/* eslint-disable */
/**
* parses text to json AST
* @param {string} s
* @param {boolean} enable_markdown
* @returns {any}
*/
export function parse_text(s: string, enable_markdown: boolean): any;
/**
* parses text to json AST (text elements and labeled links, to replicate current desktop implementation)
* @param {string} s
* @returns {any}
*/
export function parse_desktop_set(s: string): any;
/**
* returns first emoji from text if text begins with an emoji
* @param {string} input
* @returns {string | undefined}
*/
export function get_first_emoji(input: string): string | undefined;
/**
* If string contains only emojis count the emojis otherwise retuns null
* @param {string} input
* @returns {number | undefined}
*/
export function count_emojis_if_only_contains_emoji(input: string): number | undefined;
/**
* encode a host to punycode encoded string
* @param {string} host
* @returns {string}
*/
export function punycode_encode_host(host: string): string;
/**
* Returns host as decoded unicode string
* @param {string} host
* @returns {string}
*/
export function punycode_decode_host(host: string): string;
/**
* Returns true if host string contains non ASCII characters
* @param {string} host
* @returns {boolean}
*/
export function is_puny(host: string): boolean;

export type PunycodeWarning = {
  original_hostname: string;
  ascii_hostname: string;
  punycode_encoded_url: string;
};
export type LinkDestination = {
  target: string;
  hostname: null | string;
  punycode: null | PunycodeWarning;
  scheme: null | string;
};
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
  | { t: "BotCommandSuggestion"; c: string }
  | { t: "Link"; c: { destination: LinkDestination } }
  | {
      t: "LabeledLink";
      c: { label: ParsedElement[]; destination: LinkDestination };
    };



export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly parse_text: (a: number, b: number, c: number) => number;
  readonly parse_desktop_set: (a: number, b: number) => number;
  readonly get_first_emoji: (a: number, b: number, c: number) => void;
  readonly count_emojis_if_only_contains_emoji: (a: number, b: number, c: number) => void;
  readonly punycode_encode_host: (a: number, b: number, c: number) => void;
  readonly punycode_decode_host: (a: number, b: number, c: number) => void;
  readonly is_puny: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
