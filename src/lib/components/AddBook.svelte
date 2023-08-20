<script lang="ts">
  import { addBook } from "../../services/books";
  import { Button } from "../ui/button";

  let title = "";
  let author = "";
  let error: string | null;

  async function onSubmit(){
    ({ error } = await addBook({ title, author }));
  }
</script>

<form class="grid gap-2 mx-auto" on:submit|preventDefault={onSubmit}>
  <input class="p-3 bg-zinc-800 rounded-sm placeholder-slate-100 font-medium text-sm" placeholder="Title" bind:value={title} />
  <input class="p-3 bg-zinc-800 rounded-sm placeholder-slate-100 font-medium text-sm" placeholder="Author" bind:value={author} />
  <Button size="lg" class="mt-2" type="submit">Add Book</Button>
</form>

{#if error}
  <p>{error}</p>
{/if}