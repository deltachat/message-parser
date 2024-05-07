//@ts-check

import init, {
  parse_desktop_set,
  parse_text,
  get_first_emoji,
  count_emojis_if_only_contains_emoji,
} from "./pkg/message_parser_wasm.js";

/** @typedef {import("./pkg/message_parser_wasm.js").ParsedElement} ParsedElement */

/**
 *
 * @param {Node} base where the elements should get attached to
 * @param {ParsedElement[]} elms
 * @throws if element type is not known / unimplemented
 */
function renderElements(base, elms) {
  for (const elem of elms) {
    base.appendChild(renderElement(elem));
  }
}

/**
 *
 * @param {ParsedElement} elm
 * @returns {Node}
 * @throws if element type is not known / unimplemented
 */
function renderElement(elm) {
  switch (elm.t) {
    case "CodeBlock":
      let cb = document.createElement("pre");
      cb.className = "code-block";
      if (elm.c.language) {
        let lang_hint = document.createElement("span");
        lang_hint.innerText = elm.c.language;
        cb.append(lang_hint);
      }
      let innerCb = document.createElement("code");
      innerCb.innerText = elm.c.content;
      cb.append(innerCb);
      return cb;

    case "InlineCode":
      let ic = document.createElement("code");
      ic.innerText = elm.c.content;
      return ic;

    case "StrikeThrough":
      let s = document.createElement("s");
      renderElements(s, elm.c);
      return s;
    case "Italics":
      let i = document.createElement("i");
      renderElements(i, elm.c);
      return i;
    case "Bold":
      let b = document.createElement("b");
      renderElements(b, elm.c);
      return b;

    case "Tag":
      let tag = document.createElement("a");
      tag.innerText = elm.c;
      tag.href = "#";
      tag.onclick = () =>
        alert(
          `Clicked on a hastag, this should open search for the text "${elm.c}"`
        );
      return tag;

    case "Link":
      let link = document.createElement("a");
      link.innerText = elm.c.destination.target;
      link.href = elm.c.destination.target;
      return link;

    case "LabeledLink":
      let labeled_link = document.createElement("a");
      renderElements(labeled_link, elm.c.label);
      labeled_link.href = elm.c.destination.target;
      return labeled_link;

    case "EmailAddress":
      let email = document.createElement("a");
      email.innerText = elm.c;
      email.href = "mailto:" + elm.c;
      return email;

    case "BotCommandSuggestion":
      let bcs = document.createElement("a");
      bcs.innerText = elm.c;
      bcs.href = "#";
      bcs.onclick = () =>
        alert(
          `Clicked on a BotCommandSuggestion, this should replace the current draft and if the draft is not empty it should ask whether it should be replaced"${elm.c}"`
        );
      return bcs;

    case "Linebreak":
      return document.createElement("br");

    case "Text":
      let t = document.createTextNode(elm.c);
      return t;
  }
  console.error(`type ${elm.t} not known/implemented yet`, elm);
  let errElement = document.createElement("span");
  errElement.style.color = "red";
  errElement.innerText = JSON.stringify(elm);
  return errElement;
}

init().then(() => {
  console.log(parse_text);
  /** @type {HTMLTextAreaElement} */
  const input = document.getElementById("input");
  const output = document.getElementById("result");
  const output_ast = document.getElementById("ast");
  /** @type {HTMLSelectElement} */
  const parse_mode = document.getElementById("parse_mode");
  parse_mode.value = localStorage.getItem("lastMode") || "text";

  let running = false;
  let should_run_again = false;

  const action = () => {
    if (running) {
      should_run_again = true;
      return;
    }
    running = true;

    /** @type {'text'|'desktop'|'markdown'} */
    //@ts-ignore
    const mode = parse_mode.value;

    /** @type {ParsedElement[]} */
    let parsed = [];

    switch (mode) {
      case "desktop":
        parsed = parse_desktop_set(input.value);
        break;
      case "markdown":
        parsed = parse_text(input.value, true);
        break;
      case "text":
      default:
        parsed = parse_text(input.value, false);
        break;
    }

    // console.log(parsed);

    output.innerText = "";
    output_ast.innerText = JSON.stringify(parsed, null, 4);

    renderElements(output, parsed);
    running = false;
    if (should_run_again) {
      should_run_again = false;
      action();
    }
  };
  action();

  input.onkeyup = action;
  parse_mode.onchange = () => {
    localStorage.setItem("lastMode", parse_mode.value);
    action();
  };

  // emoji helpers
  /** @type {HTMLInputElement} */
  const emoji_input = document.getElementById("emoji-test");
  const emoji_out_first = document.getElementById("emoji-test-first");
  const emoji_out_count = document.getElementById("emoji-test-count");
  const emoji_update = () => {
    const text = emoji_input.value;
    emoji_out_first.innerText = String(get_first_emoji(text));
    emoji_out_count.innerText = String(
      count_emojis_if_only_contains_emoji(text)
    );
    setTimeout(emoji_update, 1)
  };
  emoji_input.onchange = emoji_input.onkeydown = ()=>setTimeout(emoji_update, 1);
});
