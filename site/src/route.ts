export function moveFocusToHeading(heading: HTMLElement, scroll = false): void {
  const status = document.querySelector<HTMLElement>("#route-status");
  window.requestAnimationFrame(() => {
    if (scroll) heading.scrollIntoView({ block: "start" });
    heading.focus({ preventScroll: true });
    if (status) {
      status.textContent = "";
      window.requestAnimationFrame(() => { status.textContent = heading.textContent?.trim() ?? ""; });
    }
  });
}

if (document.body.dataset.focusHeading === "true") {
  const heading = document.querySelector<HTMLElement>("h1");
  if (heading) moveFocusToHeading(heading);
}
