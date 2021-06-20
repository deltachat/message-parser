//@ts-check

import init, { parse } from "./pkg/message_parser_wasm.js";

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
      tag.innerText = "#" + elm.c;
      tag.href = "#";
      tag.onclick = () =>
        alert(
          `Clicked on a hastag, this should open search for the text "${
            "#" + elm.c
          }"`
        );
      return tag;

    case "Link":
      let link = document.createElement("a");
      link.innerText = elm.c.destination;
      link.href = elm.c.destination;
      return link;

    case "LabeledLink":
      let labeled_link = document.createElement("a");
      renderElements(labeled_link, elm.c.label);
      labeled_link.href = elm.c.destination;
      return labeled_link;

    case "EmailAddress":
      let email = document.createElement("a");
      email.innerText = elm.c;
      email.href = "mailto:" + elm.c;
      return email;

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
  console.log(parse);
  /** @type {HTMLTextAreaElement} */
  const input = document.getElementById("input");
  const output = document.getElementById("result");

  let running = false;
  let should_run_again = false;

  const action = () => {
    if (running) {
      should_run_again = true;
      return;
    }
    running = true;

    /** @type {ParsedElement[]} */
    let parsed = parse(input.value);
    // console.log(parsed);

    output.innerText = "";

    renderElements(output, parsed);
    running = false;
    if (should_run_again) {
      should_run_again = false;
      action();
    }
  };
  action();

  input.onkeyup = action;
});
