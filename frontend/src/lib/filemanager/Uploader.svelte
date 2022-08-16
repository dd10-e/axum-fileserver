<style global>
	@import 'filepond/dist/filepond.css';
</style>

<script>
    import { invalidate } from '$app/navigation';
    import { page } from '$app/stores';
    import { baseApi } from '$lib/services/_server.js';
	import FilePond, { registerPlugin } from 'svelte-filepond';
    import FilePondPluginImageExifOrientation from 'filepond-plugin-image-exif-orientation';
    // import fr_FR from 'filepond/locale/fr-fr.js'; / TODO: fail on production

	export let project;

	let pond;
    let fr_FR = {
        labelIdle: 'Faites glisser vos fichiers ou <span class = "filepond--label-action"> Parcourir <span>',
        labelInvalidField: "Le champ contient des fichiers invalides",
        labelFileWaitingForSize: "En attente de taille",
        labelFileSizeNotAvailable: "Taille non disponible",
        labelFileLoading: "Chargement",
        labelFileLoadError: "Erreur durant le chargement",
        labelFileProcessing: "Traitement",
        labelFileProcessingComplete: "Traitement effectué",
        labelFileProcessingAborted: "Traitement interrompu",
        labelFileProcessingError: "Erreur durant le traitement",
        labelFileProcessingRevertError: "Erreur durant la restauration",
        labelFileRemoveError: "Erreur durant la suppression",
        labelTapToCancel: "appuyez pour annuler",
        labelTapToRetry: "appuyer pour réessayer",
        labelTapToUndo: "appuyer pour revenir en arrière",
        labelButtonRemoveItem: "Retirer",
        labelButtonAbortItemLoad: "Annuler",
        labelButtonRetryItemLoad: "Recommencer",
        labelButtonAbortItemProcessing: "Annuler",
        labelButtonUndoItemProcessing: "Retour en arrière",
        labelButtonRetryItemProcessing: "Recommencer",
        labelButtonProcessItem: "Charger",
        labelMaxFileSizeExceeded: "Le fichier est trop volumineux",
        labelMaxFileSize: "La taille maximale de fichier est {filesize}",
        labelMaxTotalFileSizeExceeded: "Taille totale maximale dépassée",
        labelMaxTotalFileSize: "La taille totale maximale des fichiers est {filesize}",
        labelFileTypeNotAllowed: "Fichier non valide",
        fileValidateTypeLabelExpectedTypes: "Attendez {allButLastType} ou {lastType}",
        imageValidateSizeLabelFormatError: "Type d'image non pris en charge",
        imageValidateSizeLabelImageSizeTooSmall: "L'image est trop petite",
        imageValidateSizeLabelImageSizeTooBig: "L'image est trop grande",
        imageValidateSizeLabelExpectedMinSize: "La taille minimale est {minWidth} × {minHeight}",
        imageValidateSizeLabelExpectedMaxSize: "La taille maximale est {maxWidth} × {maxHeight}",
        imageValidateSizeLabelImageResolutionTooLow: "La résolution est trop faible",
        imageValidateSizeLabelImageResolutionTooHigh: "La résolution est trop élevée",
        imageValidateSizeLabelExpectedMinResolution: "La résolution minimale est {minResolution}",
        imageValidateSizeLabelExpectedMaxResolution: "La résolution maximale est {maxResolution}",
    };

    registerPlugin(FilePondPluginImageExifOrientation);
</script>

<div class="w-full p-4">
    <FilePond 
        name="filepond"
        {...fr_FR}
        server={{
            // Custom processing method is required
            // because Content-Type is sent twice by default in Filepond because of metadata and file object
            // Look at: https://github.com/pqina/filepond/issues/120#issuecomment-423053864
            process: (fieldName, file, metadata, load, error, progress, abort) => {
                // We ignore the metadata property and only send the file

                const formData = new FormData();
                formData.append(fieldName, file, file.name);

                const request = new XMLHttpRequest();
                request.open('POST', `${baseApi}/fileserver/documents/${project.uuid}/upload-file/${$page.params.index}`);

                request.upload.onprogress = (e) => {
                    progress(e.lengthComputable, e.loaded, e.total);
                };

                request.onload = function() {
                    if (request.status >= 200 && request.status < 300) {
                        load(request.responseText);
                    }
                    else {
                        error('oh no');
                    }
                };

                request.send(formData);

                invalidate(`/projects/${project.uuid}/documents/${$page.params.index}`)
            }
        }}
        allowMultiple={true}
        credits={false}
    />
</div>