<script>
    import "../app.css";

    document.onkeydown = function (e) {
        if (e.key === "F5") {
            return false;
        }
    };

    document.oncontextmenu = function (e) {
        e.preventDefault();
    };
</script>

<main class="w-full h-screen bg-slate-800 flex">
    <slot />
</main>
