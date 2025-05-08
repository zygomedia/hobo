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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> hobo</a></li><li class="chapter-item expanded "><a href="getting-started.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li class="chapter-item expanded "><a href="basic-concepts.html"><strong aria-hidden="true">3.</strong> Core Concepts</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="basic-concepts/entities-components-resources.html"><strong aria-hidden="true">3.1.</strong> Entities, Components (and Resources)</a></li><li class="chapter-item expanded "><a href="basic-concepts/hobo-create.html"><strong aria-hidden="true">3.2.</strong> hobo::create</a></li><li class="chapter-item expanded "><a href="basic-concepts/children-and-parent.html"><strong aria-hidden="true">3.3.</strong> Children and Parent</a></li><li class="chapter-item expanded "><a href="basic-concepts/removing-and-replacing-elements.html"><strong aria-hidden="true">3.4.</strong> Removing and replacing elements</a></li><li class="chapter-item expanded "><a href="basic-concepts/dom-events.html"><strong aria-hidden="true">3.5.</strong> DOM Events and EventHandlerCallback</a></li><li class="chapter-item expanded "><a href="basic-concepts/borrowing-and-storage.html"><strong aria-hidden="true">3.6.</strong> Borrowing and Storage</a></li></ol></li><li class="chapter-item expanded "><a href="styling.html"><strong aria-hidden="true">4.</strong> Styling</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="styling/selector.html"><strong aria-hidden="true">4.1.</strong> Selector</a></li><li class="chapter-item expanded "><a href="styling/property.html"><strong aria-hidden="true">4.2.</strong> Property</a></li><li class="chapter-item expanded "><a href="styling/at-rules.html"><strong aria-hidden="true">4.3.</strong> @-rules</a></li><li class="chapter-item expanded "><a href="styling/colors.html"><strong aria-hidden="true">4.4.</strong> Colors</a></li><li class="chapter-item expanded "><a href="styling/every-way-to-make-a-class.html"><strong aria-hidden="true">4.5.</strong> Every way to make a class</a></li></ol></li><li class="chapter-item expanded "><a href="state.html"><strong aria-hidden="true">5.</strong> Managing state and relations with Components</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="state/queries.html"><strong aria-hidden="true">5.1.</strong> Queries</a></li><li class="chapter-item expanded "><a href="state/signals.html"><strong aria-hidden="true">5.2.</strong> Signals</a></li></ol></li><li class="chapter-item expanded "><a href="building-the-dom.html"><strong aria-hidden="true">6.</strong> Building the DOM</a></li><li class="chapter-item expanded "><a href="utilities.html"><strong aria-hidden="true">7.</strong> Other utilities</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="utilities/web_str.html"><strong aria-hidden="true">7.1.</strong> web_str</a></li><li class="chapter-item expanded "><a href="utilities/events.html"><strong aria-hidden="true">7.2.</strong> Events</a></li></ol></li><li class="chapter-item expanded "><a href="recipes.html"><strong aria-hidden="true">8.</strong> Recipes</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="recipes/logging.html"><strong aria-hidden="true">8.1.</strong> Logging</a></li><li class="chapter-item expanded "><a href="recipes/elements-that-change.html"><strong aria-hidden="true">8.2.</strong> Elements that change</a></li><li class="chapter-item expanded "><a href="recipes/svgs.html"><strong aria-hidden="true">8.3.</strong> SVGs</a></li><li class="chapter-item expanded "><a href="recipes/async-and-is-dead.html"><strong aria-hidden="true">8.4.</strong> Async and .is_dead()</a></li></ol></li><li class="chapter-item expanded "><a href="hobo-plus.html"><strong aria-hidden="true">9.</strong> hobo-plus</a></li></ol>';
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
