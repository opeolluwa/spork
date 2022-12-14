
const { createApp } = Vue
createApp({
    data() {
        return {
            message: 'Hello Vue!',
            keyword: null,
            response: null,
            errorMessage: null,
            loading: false,
            response: null,
            hasError: false
        }
    },
    methods: {
        async search() {
            this.loading = true;
            try {
                const { keyword } = this;
                if (!keyword) {
                    this.hasError = true;
                    this.errorMessage = "Please enter a word"
                    return
                }
                const response = await axios.post("/api/search", { keyword: keyword.trim() })
                if (response && response.data) {
                    this.loading = false;
                    this.response = response.data[0];
                    return;
                }

                //in no response
                if (response && !response.data) {
                    this.loading = false;
                    this.hasError = true;
                    this.errorMessage = "No response from server"
                    return;
                }
            }
            catch (error) {
                console.log(error)
               /*  this.errorMessage = error.response.data.message
                return */
            }
        },
    },
    computed: {
        word() {
            return this.response.word;
        },
        phonetic() {
            return this.response.phonetic;
        },
        meanings() {
            return this.response.meanings;
        },
        definitions() {
            return this.response.meaning.definitions
        }
    }
}).mount('#app')