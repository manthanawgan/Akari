<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, tick } from "svelte";
  import TodoItem from "./lib/TodoItem.svelte";
  import type { Todo } from "./app";

  let todos = $state<Todo[]>([]);
  let draft = $state("");
  let loading = $state(true);
  let error = $state("");
  let input: HTMLInputElement;

  const incompleteCount = $derived(todos.filter((todo) => !todo.done).length);
  const sortedTodos = $derived(
    [...todos].sort((a, b) => {
      if (a.done !== b.done) return Number(a.done) - Number(b.done);
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    })
  );

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

    error = "";

    try {
      await invoke<Todo>("add_todo", { text });
      todos = await invoke<Todo[]>("get_todos");
      draft = "";
      await tick();
      input?.focus();
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  async function toggleTodo(id: string) {
    error = "";

    try {
      await invoke<Todo>("toggle_todo", { id });
      todos = await invoke<Todo[]>("get_todos");
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  async function deleteTodo(id: string) {
    error = "";

    try {
      await invoke("delete_todo", { id });
      todos = await invoke<Todo[]>("get_todos");
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  async function hideWindow() {
    await getCurrentWindow().hide();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      event.preventDefault();
      hideWindow();
    }
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    addTodo();
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
      <p>{incompleteCount} open</p>
    </div>
    <button class="icon-button" type="button" aria-label="Close Akari" onclick={hideWindow}>×</button>
  </header>

  <section class="todo-list" aria-label="Todos">
    {#if loading}
      <p class="empty">Loading...</p>
    {:else if sortedTodos.length === 0}
      <p class="empty">Nothing pending.</p>
    {:else}
      {#each sortedTodos as todo (todo.id)}
        <TodoItem {todo} onToggle={toggleTodo} onDelete={deleteTodo} />
      {/each}
    {/if}
  </section>

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  <form class="composer" onsubmit={handleSubmit}>
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
