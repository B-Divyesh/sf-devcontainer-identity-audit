import { AuditInputError, evaluateAudit, type AuditInput, type Runtime, type UserNamespace } from "./audit";

const select = <T extends Element>(selector: string): T => {
  const element = document.querySelector<T>(selector);
  if (!element) throw new Error(`Missing required element: ${selector}`);
  return element;
};

const form = select<HTMLFormElement>("#audit-form");
const result = select<HTMLElement>("#audit-result");
const error = select<HTMLElement>("#form-error");
const runtime = select<HTMLSelectElement>("#runtime");
const userns = select<HTMLSelectElement>("#userns");
const usernsLabel = select<HTMLElement>("#userns-label");

function value(id: string): string {
  return select<HTMLInputElement>(`#${id}`).value;
}

function auditInput(): AuditInput {
  return {
    ownerUid: value("owner-uid"),
    ownerGid: value("owner-gid"),
    hostUid: value("host-uid"),
    hostGid: value("host-gid"),
    subuidStart: value("subuid-start"),
    subgidStart: value("subgid-start"),
    remoteUid: value("remote-uid"),
    remoteGid: value("remote-gid"),
    mode: value("mode"),
    runtime: runtime.value as Runtime,
    userns: userns.value as UserNamespace,
    readOnly: select<HTMLInputElement>("#read-only").checked
  };
}

function runAudit(): void {
  error.hidden = true;
  try {
    const report = evaluateAudit(auditInput());
    result.className = `audit-result is-${report.verdict}`;
    select("#status-stamp").textContent = report.verdict;
    select("#result-title").textContent = report.verdict === "pass" ? "Workspace is writable" : "Mount mismatch predicted";
    select("#result-summary").textContent = report.summary;
    select("#container-id").textContent = report.containerIdentity;
    select("#mapped-id").textContent = report.mappedIdentity;
    select("#workspace-id").textContent = report.workspaceIdentity;
    select("#access-id").textContent = report.access;
    select("#remedy-text").textContent = report.remedy;
    select<HTMLElement>("#identity-grid").hidden = false;
    select<HTMLElement>("#remedy").hidden = false;
  } catch (caught) {
    result.className = "audit-result is-error";
    error.textContent = caught instanceof AuditInputError ? caught.message : "The demo could not complete. Reload and try again.";
    error.hidden = false;
    error.focus();
  }
}

function loadMismatchSample(): void {
  select<HTMLInputElement>("#owner-uid").value = "1000";
  select<HTMLInputElement>("#owner-gid").value = "1000";
  select<HTMLInputElement>("#remote-uid").value = "1000";
  select<HTMLInputElement>("#remote-gid").value = "1000";
  select<HTMLInputElement>("#host-uid").value = "1000";
  select<HTMLInputElement>("#host-gid").value = "1000";
  select<HTMLInputElement>("#subuid-start").value = "100000";
  select<HTMLInputElement>("#subgid-start").value = "100000";
  select<HTMLInputElement>("#mode").value = "0755";
  runtime.value = "podman";
  userns.value = "default";
  select<HTMLInputElement>("#read-only").checked = false;
  updateRuntimeFields();
  runAudit();
}

form.addEventListener("submit", (event) => {
  event.preventDefault();
  runAudit();
});

select("#load-safe").addEventListener("click", () => {
  select<HTMLInputElement>("#owner-uid").value = "1000";
  select<HTMLInputElement>("#owner-gid").value = "1000";
  select<HTMLInputElement>("#remote-uid").value = "1000";
  select<HTMLInputElement>("#remote-gid").value = "1000";
  select<HTMLInputElement>("#host-uid").value = "1000";
  select<HTMLInputElement>("#host-gid").value = "1000";
  select<HTMLInputElement>("#subuid-start").value = "100000";
  select<HTMLInputElement>("#subgid-start").value = "100000";
  select<HTMLInputElement>("#mode").value = "0755";
  runtime.value = "podman";
  userns.value = "keep-id";
  select<HTMLInputElement>("#read-only").checked = false;
  updateRuntimeFields();
  runAudit();
});

document.querySelector<HTMLButtonElement>("#reset-demo")?.addEventListener("click", () => {
  loadMismatchSample();
  result.focus();
});

function updateRuntimeFields(): void {
  const isPodman = runtime.value === "podman";
  usernsLabel.hidden = !isPodman;
  userns.disabled = !isPodman;
  const usesRootlessMap = isPodman && userns.value !== "host";
  const mapFields = select<HTMLFieldSetElement>("#rootless-map");
  mapFields.hidden = !usesRootlessMap;
  mapFields.querySelectorAll<HTMLInputElement>("input").forEach((input) => {
    input.disabled = !usesRootlessMap;
  });
}
runtime.addEventListener("change", updateRuntimeFields);
userns.addEventListener("change", updateRuntimeFields);
updateRuntimeFields();

if (document.body.dataset.demo === "true") {
  loadMismatchSample();
}

document.querySelectorAll<HTMLButtonElement>("[data-copy]").forEach((button) => {
  button.addEventListener("click", async () => {
    const label = button.querySelector<HTMLElement>(".copy-label");
    try {
      await navigator.clipboard.writeText(button.dataset.copy ?? "");
      if (label) label.textContent = "Copied";
    } catch {
      if (label) label.textContent = "Select command";
    }
    window.setTimeout(() => { if (label) label.textContent = "Copy"; }, 1800);
  });
});

const offlineBanner = select<HTMLElement>("#offline-banner");
const updateConnection = (): void => { offlineBanner.hidden = navigator.onLine; };
window.addEventListener("online", updateConnection);
window.addEventListener("offline", updateConnection);
updateConnection();

if ("serviceWorker" in navigator && import.meta.env.PROD) {
  window.addEventListener("load", () => { void navigator.serviceWorker.register("/sw.js"); });
}
