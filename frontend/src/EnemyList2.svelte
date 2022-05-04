<script lang="ts">
  import EnemyIcon from "./EnemyIcon.svelte";
</script>

<main>
  <div>
    <EnemyIcon name="bat" showDeaths={false} />
    <EnemyIcon name="snake" />
    <EnemyIcon name="spider" />
    <EnemyIcon name="giant_spider" />
    <EnemyIcon name="caveman" />
  </div>
  <div />
  <div>
    <EnemyIcon name="zombie" showDeaths={false} />
    <EnemyIcon name="vampire" showDeaths={false} />
    <EnemyIcon name="frog" />
    <EnemyIcon name="fire_frog" />
  </div>
  <div>
    <EnemyIcon name="monkey" showDeaths={false} />
    <EnemyIcon name="mantrap" />
    <EnemyIcon name="piranha" />
    <EnemyIcon name="megamouth" showDeaths={false} />
  </div>
  <div>
    <EnemyIcon name="yeti" />
    <EnemyIcon name="yeti_king" />
    <EnemyIcon name="alien" />
    <EnemyIcon name="ufo" />
    <EnemyIcon name="alien_boss" />
  </div>
  <div>
    <EnemyIcon name="hawkman" />
    <EnemyIcon name="tomb_lord" />
    <EnemyIcon name="shopkeeper" />
    <EnemyIcon name="magma_man" showKills={false} />
    <EnemyIcon name="olmec" />
  </div>
  <div>
    <EnemyIcon name="skeleton" />
    <EnemyIcon name="ghost" showKills={false} />
  </div>
</main>

<style>
  main {
    padding: 0.1em;
    margin: 0 auto;
  }
</style>
