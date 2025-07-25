/*
 * OpenAPI Petstore
 * This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

package org.openapitools.client.api;

import java.util.Objects;
import org.openapitools.client.ApiResponse;

import com.fasterxml.jackson.databind.ObjectMapper;

import io.helidon.common.GenericType;
import io.helidon.common.media.type.MediaTypes;
import io.helidon.config.Config;
import io.helidon.http.Method;
import io.helidon.http.media.MediaSupport;

import io.helidon.http.media.jackson.JacksonSupport;
import io.helidon.webclient.api.HttpClientRequest;
import io.helidon.webclient.api.HttpClientResponse;

import org.openapitools.client.ApiClient;

import org.openapitools.client.model.Client;
import java.util.List;
import java.util.Map;

/**
 * OpenAPI Petstore
 *
 * <p>This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 */
public class FakeClassnameTags123ApiImpl implements FakeClassnameTags123Api {

  private final ApiClient apiClient;

  protected static final GenericType<Client> RESPONSE_TYPE_testClassname = ResponseType.create(Client.class);

  /**
   * Creates a new instance of FakeClassnameTags123ApiImpl initialized with the specified {@link ApiClient}.
   *
   */
  public static FakeClassnameTags123ApiImpl create(ApiClient apiClient) {
    return new FakeClassnameTags123ApiImpl(apiClient);
  }

  protected FakeClassnameTags123ApiImpl(ApiClient apiClient) {
    this.apiClient = apiClient;
  }

  @Override
  public ApiResponse<Client> testClassname(Client client) {
    Objects.requireNonNull(client, "Required parameter 'client' not specified");
    HttpClientRequest webClientRequestBuilder = testClassnameRequestBuilder(client);
    return testClassnameSubmit(webClientRequestBuilder, client);
  }

  /**
   * Creates a {@code WebClientRequestBuilder} for the testClassname operation.
   * Optional customization point for subclasses.
   *
   * @param client client model (required)
   * @return HttpClientRequest for testClassname
   */
  protected HttpClientRequest testClassnameRequestBuilder(Client client) {
    HttpClientRequest webClientRequestBuilder = apiClient.webClient()
            .method(Method.PATCH);

    webClientRequestBuilder.path("/fake_classname_test");
    webClientRequestBuilder.contentType(MediaTypes.APPLICATION_JSON);
    webClientRequestBuilder.accept(MediaTypes.APPLICATION_JSON);

    return webClientRequestBuilder;
  }

  /**
   * Initiates the request for the testClassname operation.
   * Optional customization point for subclasses.
   *
   * @param webClientRequestBuilder the request builder to use for submitting the request
   * @param client client model (required)
   * @return {@code ApiResponse<Client>} for the submitted request
   */
  protected ApiResponse<Client> testClassnameSubmit(HttpClientRequest webClientRequestBuilder, Client client) {
    HttpClientResponse webClientResponse = webClientRequestBuilder.submit(client);
    return ApiResponse.create(RESPONSE_TYPE_testClassname, webClientResponse);
  }

}
