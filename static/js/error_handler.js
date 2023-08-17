function createElement(tagName, callback = undefined, childrens = undefined) {
  const element = document.createElement(tagName)
  callback && callback(element)
  if (childrens) {
    if (Array.isArray(childrens)) childrens.map(children => element.appendChild(children))
    else element.appendChild(childrens)
  }
  return element
}

class ErrorMessage extends HTMLElement {
  message;

  constructor(message) {
    super();
    this.attachShadow({ mode: 'open' });
    this.message = message;
  }

  get styles() {
    return `
        div {
          padding: 5px 10px;
          border-radius: 4px;
          position: absolute;
          top: 5px;
          left: 0px;
          right: 0px;
          background-color: #e86565;
          color: white;
          width: clamp(100px, 90%, 400px);
          margin: 0 auto;
        }
        div > span {
          top: 4px;
          right: 8px;
          position: absolute;
        }
        div > span:hover {
          cursor: pointer;
          font-weight: bold;
        }
    `
  }

  connectedCallback() {
    if (!this.shadowRoot) return;
    this.shadowRoot.appendChild(createElement("style", (e) => e.innerText = this.styles))

    const ErrorMessageElement = createElement("div", undefined, [
      createElement("p", (e) => e.innerText = this.message),
      createElement("span", (e) => {
        e.innerText = "✕";
        e.addEventListener("click", (_) => this.remove())
      })
    ])
    this.shadowRoot.appendChild(ErrorMessageElement);
  }
}

customElements.define('error-message', ErrorMessage)

const htmxEvents = {
  /**send this event to an element to abort a request*/
  abort: "htmx:abort",
  /**triggered after an AJAX request has completed processing a successful response*/
  afterOnLoad: "htmx:afterOnLoad",
  /**triggered after htmx has initialized a node*/
  afterProcessNode: "htmx:afterProcessNode",
  /**triggered after an AJAX request has completed*/
  afterRequest: "htmx:afterRequest",
  /**triggered after the DOM has settled*/
  afterSettle: "htmx:afterSettle",
  /**triggered after new content has been swapped in*/
  afterSwap: "htmx:afterSwap",
  /**triggered before any response processing occurs*/
  beforeOnLoad: "htmx:beforeOnLoad",
  /**triggered before htmx initializes a node*/
  beforeProcessNode: "htmx:beforeProcessNode",
  /**triggered before an AJAX request is made*/
  beforeRequest: "htmx:beforeRequest",
  /**triggered before a swap is done, allows you to configure the swap*/
  beforeSwap: "htmx:beforeSwap",
  /**triggered just before an ajax request is sent*/
  beforeSend: "htmx:beforeSend",
  /**triggered before the request, allows you to customize parameters, headers*/
  configRequest: "htmx:configRequest",
  /**triggered after a trigger occurs on an element, allows you to cancel (or delay) issuing the AJAX request*/
  confirm: "htmx:confirm",
  /**triggered on an error during cache writing*/
  historyCacheError: "htmx:historyCacheError",
  /**triggered on a cache miss in the history subsystem*/
  historyCacheMiss: "htmx:historyCacheMiss",
  /**triggered on a unsuccessful remote retrieval*/
  historyCacheMissError: "htmx:historyCacheMissError",
  /**triggered on a successful remote retrieval*/
  historyCacheMissLoad: "htmx:historyCacheMissLoad",
  /**triggered when htmx handles a history restoration action*/
  historyRestore: "htmx:historyRestore",
  /**triggered before content is saved to the history cache*/
  beforeHistorySave: "htmx:beforeHistorySave",
  /**triggered when new content is added to the DOM*/
  load: "htmx:load",
  /**triggered when an element refers to a SSE event in its trigger, but no parent SSE source has been defined*/
  noSSESourceError: "htmx:noSSESourceError",
  /**triggered when an exception occurs during the onLoad handling in htmx*/
  onLoadError: "htmx:onLoadError",
  /**triggered after an out of band element as been swapped in*/
  oobAfterSwap: "htmx:oobAfterSwap",
  /**triggered before an out of band element swap is done, allows you to configure the swap*/
  oobBeforeSwap: "htmx:oobBeforeSwap",
  /**triggered when an out of band element does not have a matching ID in the current DOM*/
  oobErrorNoTarget: "htmx:oobErrorNoTarget",
  /**triggered after a prompt is shown*/
  prompt: "htmx:prompt",
  /**triggered after an url is pushed into history*/
  pushedIntoHistory: "htmx:pushedIntoHistory",
  /**triggered when an HTTP response error (non-200 or 300 response code) occurs*/
  responseError: "htmx:responseError",
  /**triggered when a network error prevents an HTTP request from happening*/
  sendError: "htmx:sendError",
  /**triggered when an error occurs with a SSE source*/
  sseError: "htmx:sseError",
  /**triggered when a SSE source is opened*/
  sseOpen: "htmx:sseOpen",
  /**triggered when an error occurs during the swap phase*/
  swapError: "htmx:swapError",
  /**triggered when an invalid target is specified*/
  targetError: "htmx:targetError",
  /**triggered when a request timeout occurs*/
  timeout: "htmx:timeout",
  /**triggered before an element is validated*/
  validation_validate: "htmx:validation:validate",
  /**triggered when an element fails validation*/
  validation_failed: "htmx:validation:failed",
  /**triggered when a request is halted due to validation errors*/
  validation_halted: "htmx:validation:halted",
  /**triggered when an ajax request aborts*/
  xhr_abort: "htmx:xhr:abort",
  /**triggered when an ajax request ends*/
  xhr_loadend: "htmx:xhr:loadend",
  /**triggered when an ajax request starts*/
  xhr_loadstart: "htmx:xhr:loadstart",
  /**triggered periodically during an ajax request that supports progress events*/
  xhr_progress: "htmx:xhr:progress",
}

htmx.on(htmxEvents.sendError, function (event) {
  const ErrorMessageElement = new ErrorMessage("Connection failed ⊗");
  document.getElementsByTagName("body")[0].appendChild(ErrorMessageElement);
});

htmx.on(htmxEvents.responseError, function (event) {
  const ErrorMessageElement = new ErrorMessage(event.detail.xhr.response);
  document.getElementsByTagName("body")[0].appendChild(ErrorMessageElement);
});