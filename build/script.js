// Dynamically add styling to SVG elements
document.addEventListener('DOMContentLoaded', function () {
    document.querySelectorAll('span.inline-equation svg.typst-doc').forEach(function (svg) {
        if (svg instanceof SVGElement) {
            // Resizing
            svg.removeAttribute('height');
            svg.removeAttribute('width');
            svg.style.height = '0.65em';
            svg.style.width = 'auto';
        }
    });
    document.querySelectorAll('p.block-equation svg.typst-doc').forEach(function (svg) {
        if (svg instanceof SVGSVGElement) {
            // Resizing
            svg.removeAttribute('height');
            svg.removeAttribute('width');
            var viewBox = svg.viewBox.baseVal;
            var viewBoxHeight = viewBox.height;
            var blockHeight = "".concat(viewBoxHeight * 0.08, "rem");
            svg.style.height = blockHeight;
            svg.style.width = 'auto';
        }
    });
});
// Apply default, light, or dark mode
window.addEventListener("load", function (event) {
    var userPreference = localStorage.getItem("theme");
    var darkModeToggle = document.getElementById("darkModeToggle");
    if (userPreference) {
        document.body.classList.add(userPreference);
    }
    else {
        if (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) {
            document.body.classList.add("dark");
            if (darkModeToggle)
                darkModeToggle.classList.add("dark");
        }
        else {
            document.body.classList.add("light");
            if (darkModeToggle)
                darkModeToggle.classList.add("light");
        }
    }
});
function toggleDarkMode() {
    console.log("Toggling dark mode");
    var isDark = document.body.classList.contains("dark");
    document.body.classList.toggle("dark", !isDark);
    document.body.classList.toggle("light", isDark);
    var darkModeToggle = document.getElementById("darkModeToggle");
    if (darkModeToggle) {
        darkModeToggle.classList.toggle("dark", !isDark);
        darkModeToggle.classList.toggle("light", isDark);
    }
    localStorage.setItem("theme", isDark ? "light" : "dark");
}
