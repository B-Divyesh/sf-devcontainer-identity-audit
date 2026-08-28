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

form.addEventListener("submit", (event) => {
  event.preventDefault();
  runAudit();
});

select("#load-safe").addEventListener("click", () => {
  runtime.value = "podman";
  userns.value = "keep-id";
  select<HTMLInputElement>("#read-only").checked = false;
  updateRuntimeFields();
  runAudit();
});

function updateRuntimeFields(): void {
  const isPodman = runtime.value === "podman";
  usernsLabel.hidden = !isPodman;
  userns.disabled = !isPodman;
}
runtime.addEventListener("change", updateRuntimeFields);
updateRuntimeFields();

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
