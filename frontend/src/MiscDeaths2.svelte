<script lang="ts">
  import StatIcon from "./StatIcon.svelte";
</script>

<main>
  <div>
    <StatIcon collection="misc_deaths" name="rock" />
    <StatIcon collection="misc_deaths" name="explosion" />
    <StatIcon collection="misc_deaths" name="crushed" />
    <StatIcon collection="misc_deaths" name="long_fall" />
  </div>
  <div>
    <StatIcon collection="misc_deaths" name="spikes" />
    <StatIcon collection="misc_deaths" name="arrow_trap" />
    <StatIcon collection="misc_deaths" name="spear_trap" />
    <StatIcon collection="misc_deaths" name="smash_trap" />
  </div>
  <div>
    <!-- <StatIcon collection="misc_deaths" name="boulder" /> -->
    <StatIcon collection="misc_deaths" name="ceiling_trap" />
    <StatIcon collection="misc_deaths" name="pit" />
    <StatIcon collection="misc_deaths" name="lava" />
  </div>
</main>

<style>
  main {
    padding: 0.1em;
    margin: 0 auto;
  }
</style>
