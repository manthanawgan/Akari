<script lang="ts">
  import type { Todo } from "../app";

  export let todo: Todo;
  export let onToggle: (id: string) => void | Promise<void>;
  export let onDelete: (id: string) => void | Promise<void>;
</script>

<article class:done={todo.done} class="todo-item">
  <button
    class="checkbox"
    class:checked={todo.done}
    type="button"
    aria-label={todo.done ? "Mark todo incomplete" : "Mark todo done"}
    aria-pressed={todo.done}
    on:click={() => onToggle(todo.id)}
  >
    <span></span>
  </button>

  <p>{todo.text}</p>

  <button class="delete-button" type="button" aria-label="Delete todo" on:click={() => onDelete(todo.id)}>
    ×
  </button>
</article>
