// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item affix "><li class="part-title">Orientation</li><li class="chapter-item "><a href="READ-THIS-FIRST.html"><strong aria-hidden="true">1.</strong> Read This First</a></li><li class="chapter-item "><a href="WHAT-IF.html"><strong aria-hidden="true">2.</strong> What If</a></li><li class="chapter-item "><a href="BIDDER.html"><strong aria-hidden="true">3.</strong> Bidder</a></li><li class="chapter-item "><a href="GLOSSARY.html"><strong aria-hidden="true">4.</strong> Glossary</a></li><li class="chapter-item affix "><li class="part-title">System Definition</li><li class="chapter-item "><a href="CORE-SEQUENCE.html"><strong aria-hidden="true">5.</strong> Core Sequence</a></li><li class="chapter-item "><a href="ARCHITECTURE.html"><strong aria-hidden="true">6.</strong> Architecture</a></li><li class="chapter-item "><a href="CATALOG.html"><strong aria-hidden="true">7.</strong> Catalog</a></li><li class="chapter-item "><a href="ENVIRONMENT-DETERMINED-RENDERING.html"><strong aria-hidden="true">8.</strong> Environment-Determined Rendering</a></li><li class="chapter-item affix "><li class="part-title">Product and Interface</li><li class="chapter-item "><a href="PRD.html"><strong aria-hidden="true">9.</strong> Product Requirements</a></li><li class="chapter-item "><a href="WEBSITE-PRD.html"><strong aria-hidden="true">10.</strong> Website PRD</a></li><li class="chapter-item "><a href="UI-SPEC.html"><strong aria-hidden="true">11.</strong> UI Specification</a></li><li class="chapter-item affix "><li class="part-title">Platform and Data</li><li class="chapter-item "><a href="PLATFORM.html"><strong aria-hidden="true">12.</strong> Platform</a></li><li class="chapter-item "><a href="DATA-MODEL.html"><strong aria-hidden="true">13.</strong> Data Model</a></li><li class="chapter-item "><a href="API-SPEC.html"><strong aria-hidden="true">14.</strong> API Specification</a></li><li class="chapter-item affix "><li class="part-title">Security and Risk</li><li class="chapter-item "><a href="THREAT-MODEL.html"><strong aria-hidden="true">15.</strong> Threat Model</a></li><li class="chapter-item "><a href="LIMITS-AND-CIRCUIT-BREAKERS.html"><strong aria-hidden="true">16.</strong> Limits and Circuit Breakers</a></li><li class="chapter-item "><a href="KEY-MANAGEMENT-POLICY.html"><strong aria-hidden="true">17.</strong> Key Management Policy</a></li><li class="chapter-item affix "><li class="part-title">Operations</li><li class="chapter-item "><a href="DEPLOYMENT.html"><strong aria-hidden="true">18.</strong> Deployment</a></li><li class="chapter-item "><a href="SECRETS-AND-CONFIG.html"><strong aria-hidden="true">19.</strong> Secrets and Configuration</a></li><li class="chapter-item "><a href="OBSERVABILITY.html"><strong aria-hidden="true">20.</strong> Observability</a></li><li class="chapter-item "><a href="OPERATIONAL-RUNBOOK.html"><strong aria-hidden="true">21.</strong> Operational Runbook</a></li><li class="chapter-item affix "><li class="part-title">Validation and Continuity</li><li class="chapter-item "><a href="TESTING.html"><strong aria-hidden="true">22.</strong> Testing</a></li><li class="chapter-item "><a href="LAUNCH-CHECKLIST.html"><strong aria-hidden="true">23.</strong> Launch Checklist</a></li><li class="chapter-item "><a href="ROADMAP.html"><strong aria-hidden="true">24.</strong> Roadmap</a></li><li class="chapter-item "><a href="POETICS.html"><strong aria-hidden="true">25.</strong> Poetics</a></li><li class="chapter-item "><a href="FAQ.html"><strong aria-hidden="true">26.</strong> FAQ</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
