<script lang="ts">
  import { addBook } from "../../services/books";

  let book = "";
  let error: string | null;

  async function onSubmit(){
    ({ error } = await addBook(book));
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={onSubmit}>
    <input id="greet-input" placeholder="Enter a book..." bind:value={book} />
    <button type="submit">Add Book</button>
  </form>

  {#if error}
    <p>{error}</p>
  {/if}
</div>