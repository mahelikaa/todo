const form = document.querySelector("#todo-form");
const formTitle = document.querySelector("#form-title");
const titleInput = document.querySelector("#title");
const descriptionInput = document.querySelector("#description");
const submitButton = document.querySelector("#submit-button");
const cancelEditButton = document.querySelector("#cancel-edit");
const todoList = document.querySelector("#todo-list");
const todoTemplate = document.querySelector("#todo-template");
const emptyState = document.querySelector("#empty-state");
const taskCount = document.querySelector("#task-count");
const message = document.querySelector("#message");
const refreshButton = document.querySelector("#refresh-button");

const noteColors = ["note-yellow", "note-pink", "note-blue", "note-green", "note-orange"];
let todos = [];
let editingId = null;
let messageTimer;

async function request(url, options = {}) {
    const response = await fetch(url, options);

    if (!response.ok) {
        let errorMessage = "Something went wrong";

        try {
            const body = await response.json();
            errorMessage = body.error || errorMessage;
        } catch {
            // The server did not return JSON.
        }

        throw new Error(errorMessage);
    }

    if (response.status === 204) {
        return null;
    }

    return response.json();
}

async function loadTodos() {
    try {
        todos = await request("/todos");
        renderTodos();
    } catch (error) {
        showMessage(error.message, true);
    }
}

function renderTodos() {
    todoList.replaceChildren();
    taskCount.textContent = todos.length;
    emptyState.classList.toggle("hidden", todos.length > 0);

    for (const todo of todos) {
        const fragment = todoTemplate.content.cloneNode(true);
        const article = fragment.querySelector(".todo-item");
        const completeButton = fragment.querySelector(".complete-button");

        article.classList.add(noteColors[(todo.id - 1) % noteColors.length]);
        article.classList.toggle("completed", todo.completed);
        fragment.querySelector(".todo-id").textContent = `NOTE ${todo.id}`;
        fragment.querySelector(".todo-title").textContent = todo.title;

        const description = fragment.querySelector(".todo-description");
        description.textContent = todo.description || "No extra details.";

        completeButton.setAttribute(
            "aria-label",
            todo.completed ? "Mark task incomplete" : "Mark task complete",
        );
        completeButton.addEventListener("click", () => toggleCompletion(todo));
        fragment.querySelector(".edit-button").addEventListener("click", () => startEdit(todo));
        fragment.querySelector(".delete-button").addEventListener("click", () => removeTodo(todo));

        todoList.append(fragment);
    }
}

async function saveTodo(event) {
    event.preventDefault();

    const title = titleInput.value.trim();
    const description = descriptionInput.value.trim() || null;

    if (!title) {
        showMessage("Give your note a title first.", true);
        return;
    }

    try {
        if (editingId === null) {
            await request("/todos", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ title, description }),
            });
            showMessage("Note pinned!");
        } else {
            await request(`/todos/${editingId}`, {
                method: "PUT",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ title, description }),
            });
            showMessage("Note updated!");
        }

        resetForm();
        await loadTodos();
    } catch (error) {
        showMessage(error.message, true);
    }
}

function startEdit(todo) {
    editingId = todo.id;
    titleInput.value = todo.title;
    descriptionInput.value = todo.description || "";
    formTitle.textContent = `Edit note ${todo.id}`;
    submitButton.textContent = "Save changes";
    cancelEditButton.classList.remove("hidden");
    titleInput.focus();
    form.scrollIntoView({ behavior: "smooth", block: "center" });
}

function resetForm() {
    editingId = null;
    form.reset();
    formTitle.textContent = "Add something";
    submitButton.textContent = "Pin this note";
    cancelEditButton.classList.add("hidden");
}

async function toggleCompletion(todo) {
    try {
        await request(`/todos/${todo.id}/complete`, {
            method: "PATCH",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ completed: !todo.completed }),
        });
        await loadTodos();
    } catch (error) {
        showMessage(error.message, true);
    }
}

async function removeTodo(todo) {
    const confirmed = window.confirm(`Remove “${todo.title}” from your board?`);
    if (!confirmed) return;

    try {
        await request(`/todos/${todo.id}`, { method: "DELETE" });
        if (editingId === todo.id) resetForm();
        showMessage("Note removed.");
        await loadTodos();
    } catch (error) {
        showMessage(error.message, true);
    }
}

function showMessage(text, isError = false) {
    window.clearTimeout(messageTimer);
    message.textContent = text;
    message.style.color = isError ? "#5e1717" : "#fffdf5";
    messageTimer = window.setTimeout(() => {
        message.textContent = "";
    }, 3000);
}

form.addEventListener("submit", saveTodo);
cancelEditButton.addEventListener("click", resetForm);
refreshButton.addEventListener("click", loadTodos);

loadTodos();
