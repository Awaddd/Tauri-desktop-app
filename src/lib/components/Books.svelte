<script lang="ts">
  import { onMount } from 'svelte';
  import { getBooks } from '../../services/books'

  let loading = false;
  let data: Book[] = []
  let error: string;

  onMount(() => {
    async function load() {
      ({ data, error } = await getBooks());

      let temp = [];

      for (let i = 0; i < 5; i++) {
        temp.push(data[i]);
      }

      temp.push(data[data.length - 1]);
      data = temp;

      loading = false;
    }

    loading = true;
    load();
  })
</script>

<div>
  {#if data && data.length > 0}
    {#each data as book}
      <li class="list-none text-center">{book.title} by {book.author} ({book.isbn})</li>
    {/each}
  {/if}

  {#if loading}
    <p class="text-center">Loading...</p>
  {/if}

  {#if error}
    <p>{error}</p>
  {/if}
</div>