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
  let listRevision = $state(0);
  let incompleteCount = $state(0);
  let input: HTMLInputElement;

  function sortTodos(nextTodos: Todo[]) {
    return [...nextTodos].sort((a, b) => {
      if (a.done !== b.done) return Number(a.done) - Number(b.done);
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });
  }

  function setTodos(nextTodos: Todo[]) {
    todos = sortTodos(nextTodos);
    incompleteCount = todos.filter((todo) => !todo.done).length;
    listRevision += 1;
  }

  async function refreshTodos() {
    setTodos(await invoke<Todo[]>("get_todos"));
  }

  async function loadTodos() {
    loading = true;
    error = "";

    try {
      await refreshTodos();
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
      setTodos(await invoke<Todo[]>("add_todo", { text }));
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
      setTodos(await invoke<Todo[]>("toggle_todo", { id }));
      await tick();
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  async function deleteTodo(id: string) {
    error = "";

    try {
      setTodos(await invoke<Todo[]>("delete_todo", { id }));
      await tick();
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
    let unlistenFocus: (() => void) | undefined;
    const refreshTimer = globalThis.setInterval(() => {
      refreshTodos();
    }, 1000);
    const appWindow = getCurrentWindow();

    loadTodos();

    appWindow.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        refreshTodos();
        tick().then(() => input?.focus());
      }
    }).then((unlisten) => {
      unlistenFocus = unlisten;
    });

    return () => {
      globalThis.clearInterval(refreshTimer);
      unlistenFocus?.();
    };
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
    {:else if todos.length === 0}
      <p class="empty">Nothing pending.</p>
    {:else}
      {#key listRevision}
        {#each todos as todo (todo.id)}
          <TodoItem {todo} onToggle={toggleTodo} onDelete={deleteTodo} />
        {/each}
      {/key}
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
