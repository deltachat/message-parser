// needs to be put into TS_APPEND_CONTENT manually, didn't find an easy way to include this file directly

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
