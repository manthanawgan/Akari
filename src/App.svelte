<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, tick } from "svelte";
  import TodoItem from "./lib/TodoItem.svelte";
  import type { Todo } from "./app";

  let todos: Todo[] = [];
  let draft = "";
  let loading = true;
  let error = "";
  let input: HTMLInputElement;

  const incompleteCount = () => todos.filter((todo) => !todo.done).length;

  const sortedTodos = () =>
    [...todos].sort((a, b) => {
      if (a.done !== b.done) return Number(a.done) - Number(b.done);
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });

  async function loadTodos() {
    loading = true;
    error = "";

    try {
      todos = await invoke<Todo[]>("get_todos");
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
      await tick();
      input?.focus();
    }
  }

  async function addTodo() {
    const text = draft.trim();
    if (!text) return;

    draft = "";
    error = "";

    try {
      const todo = await invoke<Todo>("add_todo", { text });
      todos = [todo, ...todos];
      await tick();
      input?.focus();
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      draft = text;
    }
  }

  async function toggleTodo(id: string) {
    error = "";

    try {
      const updated = await invoke<Todo>("toggle_todo", { id });
      todos = todos.map((todo) => (todo.id === id ? updated : todo));
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  async function deleteTodo(id: string) {
    error = "";
    const previous = todos;
    todos = todos.filter((todo) => todo.id !== id);

    try {
      await invoke("delete_todo", { id });
    } catch (err) {
      todos = previous;
      error = err instanceof Error ? err.message : String(err);
    }
  }

  async function closeWindow() {
    await getCurrentWindow().close();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      event.preventDefault();
      closeWindow();
    }
  }

  onMount(() => {
    loadTodos();
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<main class="akari-shell">
  <header class="titlebar">
    <div>
      <h1>Akari</h1>
      <p>{incompleteCount()} open</p>
    </div>
    <button class="icon-button" type="button" aria-label="Close Akari" on:click={closeWindow}>×</button>
  </header>

  <section class="todo-list" aria-label="Todos">
    {#if loading}
      <p class="empty">Loading...</p>
    {:else if sortedTodos().length === 0}
      <p class="empty">Nothing pending.</p>
    {:else}
      {#each sortedTodos() as todo (todo.id)}
        <TodoItem {todo} onToggle={toggleTodo} onDelete={deleteTodo} />
      {/each}
    {/if}
  </section>

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  <form class="composer" on:submit|preventDefault={addTodo}>
    <input
      bind:this={input}
      bind:value={draft}
      autocomplete="off"
      maxlength="240"
      placeholder="Add a note..."
      spellcheck="true"
    />
  </form>
</main>
