const { invoke } = window.__TAURI__.tauri;

const BASE_URL = "http://localhost:8080";

function toggle_task(id) {
    fetch(`${BASE_URL}/task/${id}`, { method: "PATCH" })
        .then((response) => response.json())
        .then((res) => {
            consle.log(`rows affected: ${res.rows_affected}`);
            const task_item = document.getElementById(id);
            task_item.classList.toggle("completed");
        });
}

// TODO: ! continue 6:11

window.addEventListener("DOMContentLoaded", () => {
});
