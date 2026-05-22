<script lang="ts">
  import type { Todo } from "../app";

  let {
    todo,
    onToggle,
    onDelete
  } = $props<{
    todo: Todo;
    onToggle: (id: string) => void | Promise<void>;
    onDelete: (id: string) => void | Promise<void>;
  }>();
</script>

<article class:done={todo.done} class="todo-item">
  <button
    class="checkbox"
    class:checked={todo.done}
    type="button"
    aria-label={todo.done ? "Mark todo incomplete" : "Mark todo done"}
    aria-pressed={todo.done}
    onclick={() => onToggle(todo.id)}
  >
    <span></span>
  </button>

  <p>{todo.text}</p>

  <button class="delete-button" type="button" aria-label="Delete todo" onclick={() => onDelete(todo.id)}>
    ×
  </button>
</article>
