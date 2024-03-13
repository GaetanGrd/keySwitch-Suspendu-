<script>    
	import './styles.css';
    import { invoke } from '@tauri-apps/api/tauri'

    async function createProfile() {
        const result = await invoke('create_default_profil')
        console.log(result)
    }
</script>

<div class="app">
	<main>
        <div>
            <button on:click={createProfile}>Create Profile</button>
        </div>
        
		<slot />
	</main>
</div>
