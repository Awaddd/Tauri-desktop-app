<script lang="ts">
  import Books from './lib/components/Books.svelte';
  import AddBook from './lib/components/AddBook.svelte'
</script>

<main class="h-screen py-8 px-20 w-screen grid grid-rows-[max-content_max-content_1fr]">
  <h1 class="text-2xl text-center font-semibold">Bookio</h1>

  <div class="mt-8">
    <Books />
  </div>

  <div class="mt-8 self-end">
    <AddBook />
  </div>
</main>
