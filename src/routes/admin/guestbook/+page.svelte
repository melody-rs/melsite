<script lang="ts">
  import Navbar from "$lib/components/navbar.svelte";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
  let date_format = new Intl.DateTimeFormat(undefined, {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
</script>

<Navbar />

<table style="margin-top: 8px;">
  <tbody>
    <tr>
      <th>Actions</th>
      <th>Id</th>
      <th>Name</th>
      <th>Text</th>
      <th>Date</th>
      <th>Website</th>
    </tr>
    {#each data.entries as entry}
      <tr>
        <td>
          <form method="POST" action="?/delete_entry">
            <input type="hidden" value={entry.id} name="entry_id" />
            <input type="submit" value="Delete" />
          </form>
        </td>
        <td>{entry.id}</td>
        <td>{entry.name}</td>
        <td>{entry.text}</td>
        <td>{date_format.format(entry.date)}</td>
        <td>{entry.website}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    font-size: 24px;
    color: black;
    width: 100%;
  }

  td {
    padding: 8px;
    border: 2px solid gray;
  }
  th {
    padding: 8px;
    border: 2px solid gray;
  }

  tr:nth-child(2n) {
    background-color: lightgray;
  }
  tr:nth-child(2n + 1) {
    background-color: white;
  }
</style>
