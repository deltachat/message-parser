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
* parses text to json AST (text elements and labled links, to replicate current desktop implementation)
* @param {string} s
* @returns {any}
*/
export function parse_desktop_set(s: string): any;

export type PunycodeWarning = {
  original_hostname: string;
  ascii_hostname: string;
  punycode_encoded_url: string;
};
export type LinkDestination = {
  target: string;
  hostname: null | string;
  punycode: null | PunycodeWarning;
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
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
