async function fetchTodos() {
    const res = await fetch('/todos');
    const todos = await res.json();
    const list = document.getElementById('list');
    list.innerHTML = '';
    todos.forEach(todo => {
        const li = document.createElement('li');
        li.textContent = todo.text + (todo.mark ? ' \u2713' : '');

        const markBtn = document.createElement('button');
        markBtn.textContent = todo.mark ? 'Unmark' : 'Mark';
        markBtn.onclick = async () => {
            await fetch(`/todos/${todo.id}/mark`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ mark: !todo.mark })
            });
            fetchTodos();
        };

        const delBtn = document.createElement('button');
        delBtn.textContent = 'Delete';
        delBtn.onclick = async () => {
            await fetch(`/todos/${todo.id}`, { method: 'DELETE' });
            fetchTodos();
        };

        li.appendChild(markBtn);
        li.appendChild(delBtn);
        list.appendChild(li);
    });
}

document.getElementById('add-form').addEventListener('submit', async e => {
    e.preventDefault();
    const text = document.getElementById('text').value;
    await fetch('/todos', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text, parent_id: null })
    });
    document.getElementById('text').value = '';
    fetchTodos();
});

fetchTodos();
