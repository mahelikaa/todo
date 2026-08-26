const form = document.querySelector("#todo-form");
const titleInput = document.querySelector("#title");
const descriptionInput = document.querySelector("#description");
const todoList = document.querySelector("#todo-list");
const emptyState = document.querySelector("#empty-state");
const taskCount = document.querySelector("#task-count");
const message = document.querySelector("#message");
const refreshButton = document.querySelector("#refresh-button");

async function loadTodos() {
    try {
        const response = await fetch("/todos");

        if (!response.ok) {
            throw new Error("Could not load tasks");
        }

        const todos = await response.json();
        renderTodos(todos);
    } catch (error) {
        showMessage(error.message);
    }
}

function renderTodos(todos) {
    todoList.innerHTML = "";
    taskCount.textContent = todos.length;
    emptyState.classList.toggle("hidden", todos.length > 0);

    for (const todo of todos) {
        const article = document.createElement("article");
        article.className = "todo-item";

        const id = document.createElement("span");
        id.className = "todo-id";
        id.textContent = `TASK ${todo.id}`;

        const title = document.createElement("h3");
        title.textContent = todo.title;

        article.append(id, title);

        if (todo.description) {
            const description = document.createElement("p");
            description.textContent = todo.description;
            article.append(description);
        }

        todoList.append(article);
    }
}

async function createTodo(event) {
    event.preventDefault();

    const title = titleInput.value.trim();
    const description = descriptionInput.value.trim();

    if (!title) {
        showMessage("Please enter a task title");
        return;
    }

    try {
        const response = await fetch("/todos", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                title,
                description: description || null,
            }),
        });

        if (!response.ok) {
            throw new Error("Could not create task");
        }

        form.reset();
        showMessage("Task added");
        await loadTodos();
        titleInput.focus();
    } catch (error) {
        showMessage(error.message);
    }
}

function showMessage(text) {
    message.textContent = text;

    window.setTimeout(() => {
        message.textContent = "";
    }, 3000);
}

form.addEventListener("submit", createTodo);
refreshButton.addEventListener("click", loadTodos);

loadTodos();