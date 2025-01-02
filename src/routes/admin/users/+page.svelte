<script lang="ts">
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
</script>

<table>
  <tbody>
    <tr>
      <th>Actions</th>
      <th>Id</th>
      <th>Username</th>
      <th>GitHub Id</th>
      <th>Is Admin</th>
      <th>Sessions</th>
    </tr>
    {#each data.users as user}
      <tr>
        <td>
          <form method="POST" action="?/delete_user">
            <input type="hidden" value={user.id} name="user_id" />
            <input type="submit" value="Delete" />
          </form>
          <form method="POST" action="?/toggle_admin">
            <input type="hidden" value={user.id} name="user_id" />
            <input type="submit" value="Toggle Admin" />
          </form>
          <form method="POST" action="?/delete_sessions">
            <input type="hidden" value={user.id} name="user_id" />
            <input type="submit" value="Delete Sessions" />
          </form>
        </td>
        <td>{user.id}</td>
        <td>{user.username}</td>
        <td>{user.github_id}</td>
        <td>{user.is_admin}</td>
        <td>{JSON.stringify(user.session)}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    font-family: "Iosevka SS05 Web", monospace;
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
